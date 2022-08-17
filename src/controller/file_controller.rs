use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use mongodb::bson::doc;

use service::{face_info_service, file_service};

use crate::entity::face_info::FaceInfo;
use crate::entity::face_info::UriType::Local;
use crate::{resource, service, utils};

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
    let mut face_info = FaceInfo::default();
    face_info.file_uri = file_uri;
    face_info.file_name = file_name;
    face_info.id = face_info_id;
    face_info.uri_type = Local;
    face_info.md5 = file_md5;
    face_info.created_on = chrono::Local::now().timestamp_millis();

    match face_info_service::add_face_info(&face_info).await {
        Ok(_) => {
            info!("Saving face info success, face_info: {:?}", face_info);
            HttpResponse::Ok().await
        }
        Err(err) => {
            error!("Failed to call add_face_info, error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    }
}
