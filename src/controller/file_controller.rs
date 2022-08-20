use actix_multipart::Multipart;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use service::{face_info_service, file_resource_service};

use crate::entity::file_resource::{FileResource, UriType};
use crate::{resource, service, utils};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFileResourceByStreamResp {
    file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFileResourceReq {
    file_resource: FileResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFileResourceResp {
    file_resource_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadFileReq {
    face_info_id: String,
}

#[post("/create_file_resource_by_stream")]
pub async fn create_file_resource_by_stream(payload: Multipart) -> Result<HttpResponse, Error> {
    info!("create_file_resource_by_stream start");

    // Step 0: Generate id
    let file_resource_id = resource::id_generator::get_id().await;

    // Step 1: Save the file & calculate md5 hash
    let file_name = match service::file_resource_service::create_file_resource_with_stream(
        payload,
        &file_resource_id,
    )
    .await
    {
        Ok(file_name) => file_name,
        Err(err) => {
            error!("Failed to save_file, error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };
    let file_uri = file_resource_service::get_local_filepath(&file_resource_id, &file_name);
    let file_md5 = match utils::md5::get_file_md5(&file_uri).await {
        Ok(file_md5) => file_md5,
        Err(err) => {
            error!("Failed to get_file_md5, error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };

    info!(
        "Saving file success, file_name: {:?}, md5: {:?}",
        file_name, file_md5
    );

    // Step 2: Check file md5 is repeated
    match face_info_service::get_one_face_info_by_doc_filter(doc! {"md5": &file_md5}).await {
        Ok(res) => match res {
            None => {
                info!("Saving file success, file_uri: {:?}", file_uri);
            }
            Some(_) => {
                error!("File has already been saved!");

                file_resource_service::delete_file(&file_uri).await;
                info!(
                    "Delete file success, file_name: {:?}, md5: {:?}",
                    file_name, file_md5
                );

                return HttpResponse::Forbidden().await;
            }
        },
        Err(err) => {
            error!("Failed to get_one_face_info_by_doc, error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };

    // Step 3：Save file_resource
    match file_resource_service::create_file_resource(&FileResource {
        id: file_resource_id.clone(),
        md5: file_md5,
        file_name,
        file_uri,
        ..FileResource::default()
    })
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json(CreateFileResourceByStreamResp {
            file_id: file_resource_id,
        })),
        Err(err) => {
            log::error!("Error: {:?}", err);
            HttpResponse::InternalServerError().await
        }
    }
}

#[post("/create_file_resource")]
pub async fn create_file_resource(
    mut req: web::Json<CreateFileResourceReq>,
) -> Result<impl Responder, Error> {
    info!("req: {:?}", &req);

    let file_resource_id = resource::id_generator::get_id().await;
    req.file_resource.id = file_resource_id;

    check_create_file_resource_req(&req.file_resource).await?;

    match file_resource_service::create_file_resource(&req.file_resource).await {
        Ok(_) => Ok(HttpResponse::Ok().json(CreateFileResourceResp {
            file_resource_id: req.file_resource.id.clone(),
        })),
        Err(err) => {
            log::error!("Error: {:?}", err);
            HttpResponse::InternalServerError().await
        }
    }
}

async fn check_create_file_resource_req(file_resource: &FileResource) -> Result<(), Error> {
    match file_resource.uri_type {
        UriType::Local => {}
        UriType::Url => {}
    };

    Ok(())
}

#[get("/download_local_file/{face_info_id}")]
pub async fn download_local_file(
    req: actix_web::HttpRequest,
    face_info_id: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    info!("req: {:?}", &req);

    let face_info_id = face_info_id.into_inner();

    if face_info_id.is_empty() {
        info!("not found face_info, face_info_id is empty");
        return HttpResponse::NotFound().await;
    }

    // Step 1: Find face info
    let face_info = match face_info_service::get_one_face_info_by_doc_filter(
        doc! {"id": &face_info_id},
    )
    .await
    {
        Ok(face_info) => match face_info {
            None => {
                info!("face_info not found, face_info_id: {:?}", face_info_id);
                return HttpResponse::NotFound().await;
            }
            Some(face_info) => face_info,
        },
        Err(err) => {
            log::error!("Error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };

    // Step 2: Get file
    let file_resource_info = match file_resource_service::get_one_file_resource_by_doc_filter(
        doc! {"id": &face_info.file_id},
    )
    .await
    {
        Ok(file_resource_info) => match file_resource_info {
            None => {
                info!(
                    "file_resource_info not found, file_id: {:?}",
                    face_info.file_id
                );
                return HttpResponse::NotFound().await;
            }
            Some(file_resource_info) => file_resource_info,
        },
        Err(err) => {
            log::error!("Error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };

    let file = actix_files::NamedFile::open_async(file_resource_info.file_uri)
        .await
        .unwrap();
    Ok(file.into_response(&req))
}
