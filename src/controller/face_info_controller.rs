use actix_web::{post, web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::entity::face_info::FaceInfo;
use crate::service::face_info_service;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRandomFaceInfoRandomlyReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRandomFaceInfoRandomlyResp {
    face_infos: Vec<FaceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFaceInfoByIdReq {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFaceInfoByIdResp {
    face_info: FaceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFaceInfoReq {
    face_info: FaceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFaceInfoResp {}

#[post("/get_face_info_randomly")]
pub async fn get_face_info_randomly(
    req: web::Json<GetRandomFaceInfoRandomlyReq>,
) -> Result<impl Responder, Error> {
    log::debug!("req: {:?}", &req);

    let face_infos = face_info_service::get_face_info_randomly(2)
        .await
        .unwrap_or_default();

    Ok(HttpResponse::Ok().json(GetRandomFaceInfoRandomlyResp { face_infos }))
}

#[post("/get_face_info_by_id")]
pub async fn get_face_info_by_id(
    req: web::Json<GetFaceInfoByIdReq>,
) -> Result<impl Responder, Error> {
    log::debug!("req: {:?}", &req);

    let face_info_id = &req.id;
    if face_info_id.is_empty() {
        return HttpResponse::NotFound().await;
    }

    let face_info = match face_info_service::get_face_info_by_id(face_info_id).await {
        Ok(face_info) => match face_info {
            None => {
                return HttpResponse::NotFound().await;
            }
            Some(face_info) => face_info,
        },
        Err(err) => {
            log::error!("Error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };
    Ok(HttpResponse::Ok().json(GetFaceInfoByIdResp { face_info }))
}

#[post("/add_face_info")]
pub async fn add_face_info(req: web::Json<AddFaceInfoReq>) -> Result<impl Responder, Error> {
    log::debug!("req: {:?}", &req);

    match face_info_service::add_face_info(&req.face_info).await {
        Ok(_) => Ok(HttpResponse::Ok().json(AddFaceInfoResp {})),
        Err(err) => {
            log::error!("Error: {:?}", err);
            HttpResponse::InternalServerError().await
        }
    }
}
