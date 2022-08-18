use actix_multipart::Multipart;
use actix_web::{get, post, web, Error, HttpResponse};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use service::{face_info_service, file_service};

use crate::entity::face_info::FaceInfo;
use crate::entity::face_info::UriType::Local;
use crate::{resource, service, utils};

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveFileResp {
    face_info_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadFileReq {
    face_info_id: String,
}

#[post("/save_file")]
pub async fn save_file(payload: Multipart) -> Result<HttpResponse, Error> {
    // Step 0: Generate id
    let face_info_id = resource::id_generator::get_id().await;

    // Step 1: Save the file & calculate md5 hash
    let file_name = match service::file_service::save_file_in_payload(payload, &face_info_id).await
    {
        Ok(file_name) => file_name,
        Err(err) => {
            error!("Failed to save_file, error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };
    let file_uri = file_service::get_local_filepath(&face_info_id, &file_name);
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
            None => {}
            Some(_) => {
                error!("File has already been saved!");

                file_service::delete_file(&file_uri).await;
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

    // Step 3: Save faceInfo
    let face_info = FaceInfo {
        file_uri,
        file_name,
        id: face_info_id,
        uri_type: Local,
        md5: file_md5,
        created_on: chrono::Local::now().timestamp_millis(),
        ..Default::default()
    };

    match face_info_service::add_face_info(&face_info).await {
        Ok(_) => {
            info!("Saving face info success, face_info: {:?}", face_info);
            Ok(HttpResponse::Ok().json(SaveFileResp {
                face_info_id: face_info.id,
            }))
        }
        Err(err) => {
            error!("Failed to call add_face_info, error: {:?}", err);
            HttpResponse::InternalServerError().await
        }
    }
}

#[get("/download_file/{face_info_id}")]
pub async fn download_file(
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
    let file_path = file_service::get_local_filepath(&face_info.id, &face_info.file_name);
    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();
    Ok(file.into_response(&req))
}
