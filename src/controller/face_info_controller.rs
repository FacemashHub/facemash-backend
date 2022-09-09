use crate::algorithm::elo_rating::{compete_uscf, EloScore, WIN};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{post, web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::dao::rating_log_dao::add_rating_logs;
use crate::entity::face_info::FaceInfo;
use crate::entity::file_resource::FileResource;
use crate::entity::rating_log::RatingLog;
use crate::resource;
use crate::service::face_info_service::update_face_info_rating;
use crate::service::{face_info_service, file_resource_service};
use crate::{doc, entity};

#[derive(Debug, Serialize, Deserialize)]
pub struct FaceAndFileResourceInfo {
    face_info: FaceInfo,
    file_resource: FileResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRandomFaceInfoRandomlyReq {
    face_info_cnt: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRandomFaceInfoRandomlyResp {
    face_and_file_infos: Vec<FaceAndFileResourceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFaceInfoByIdReq {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFaceInfoByIdResp {
    face_and_file_info: FaceAndFileResourceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFaceInfoReq {
    face_info: FaceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFaceInfoResp {
    face_info_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteFaceInfoReq {
    win_face_info_id: String,
    lose_face_info_id: String,
    voter: String,
}

#[post("/get_face_info_randomly")]
pub async fn get_face_info_randomly(
    mut req: web::Json<GetRandomFaceInfoRandomlyReq>,
) -> Result<impl Responder, Error> {
    log::debug!("req: {:?}", &req);

    if req.face_info_cnt <= 0 {
        req.face_info_cnt = 2
    }

    let face_infos = face_info_service::get_face_info_randomly(req.face_info_cnt)
        .await
        .unwrap_or_default();

    let mut face_and_file_infos = vec![];
    for face_info in face_infos {
        face_and_file_infos.push(FaceAndFileResourceInfo {
            file_resource: match file_resource_service::get_one_file_resource_by_doc_filter(
                doc! {"id": &face_info.file_id},
            )
            .await
            {
                Ok(file_info) => match file_info {
                    None => FileResource::default(),
                    Some(file_info) => file_info,
                },
                Err(err) => {
                    log::error!("Error: {:?}", err);
                    return HttpResponse::InternalServerError().await;
                }
            },
            face_info,
        });
    }

    Ok(HttpResponse::Ok().json(GetRandomFaceInfoRandomlyResp {
        face_and_file_infos,
    }))
}

#[post("/get_face_info_by_id")]
pub async fn get_face_info_by_id(
    req: web::Json<GetFaceInfoByIdReq>,
) -> Result<impl Responder, Error> {
    info!("req: {:?}", &req);

    let face_info_id = &req.id;
    if face_info_id.is_empty() {
        return HttpResponse::NotFound().await;
    }

    let face_info =
        match face_info_service::get_one_face_info_by_doc_filter(doc! {"id": face_info_id}).await {
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

    let file_resource = match file_resource_service::get_one_file_resource_by_doc_filter(
        doc! {"id": &face_info.file_id},
    )
    .await
    {
        Ok(file_info) => match file_info {
            None => FileResource::default(),
            Some(file_info) => file_info,
        },
        Err(err) => {
            log::error!("Error: {:?}", err);
            return HttpResponse::InternalServerError().await;
        }
    };

    Ok(HttpResponse::Ok().json(GetFaceInfoByIdResp {
        face_and_file_info: FaceAndFileResourceInfo {
            face_info,
            file_resource,
        },
    }))
}

#[post("/add_face_info")]
pub async fn add_face_info(mut req: web::Json<AddFaceInfoReq>) -> Result<impl Responder, Error> {
    info!("req: {:?}", &req);

    let face_info_id = resource::id_generator::get_id().await;
    req.face_info.id = face_info_id;
    req.face_info.created_on = chrono::Utc::now().timestamp();
    req.face_info.score = entity::face_info::DEFAULT_SCORE;

    check_add_face_info_param(&req.face_info).await?;

    match face_info_service::add_face_info(&req.face_info).await {
        Ok(_) => Ok(HttpResponse::Ok().json(AddFaceInfoResp {
            face_info_id: req.face_info.id.clone(),
        })),
        Err(err) => {
            log::error!("Error: {:?}", err);
            HttpResponse::InternalServerError().await
        }
    }
}

#[post("/vote_face_info")]
pub async fn vote_face_info(req: web::Json<VoteFaceInfoReq>) -> Result<impl Responder, Error> {
    info!("req: {:?}", &req);

    if req.win_face_info_id.is_empty() || req.lose_face_info_id.is_empty() {
        return Err(ErrorBadRequest("face_info_id is required!"));
    };

    // Step 1: find corresponding face_info
    let filter_doc = doc! {
        "id" :{"$in": [req.win_face_info_id.as_str(), req.lose_face_info_id.as_str()]}
    };
    let face_info_map: HashMap<String, FaceInfo> =
        match face_info_service::get_face_infos_by_doc_filter(filter_doc).await {
            Ok(res) => {
                if res.len() < 2 {
                    return Err(ErrorNotFound("FaceInfo not found!"));
                }

                let mut ret_map = HashMap::new();
                for x in res {
                    ret_map.insert(x.id.clone(), x);
                }
                ret_map
            }
            Err(err) => {
                log::error!("Error: {:?}", err);
                return HttpResponse::InternalServerError().await;
            }
        };

    // Step 2：Calculate Score
    let win_face_info = match face_info_map.get(req.win_face_info_id.as_str()) {
        None => {
            return Err(ErrorNotFound("Winner FaceInfo not found!"));
        }
        Some(win_face_info) => win_face_info,
    };
    let lose_face_info = match face_info_map.get(req.lose_face_info_id.as_str()) {
        None => {
            return Err(ErrorNotFound("Loser FaceInfo not found!"));
        }
        Some(lose_face_info) => lose_face_info,
    };

    let (win_score, lose_score) = compete_uscf(
        win_face_info.score as EloScore,
        lose_face_info.score as EloScore,
        WIN,
    );

    // Step 3：Update Score
    let now = chrono::Utc::now().timestamp();
    if let Err(err) = update_face_info_rating(
        &win_face_info.id,
        win_score as f64,
        true,
        req.voter.as_str(),
        now,
    )
    .await
    {
        log::error!("Error: {:?}", err);
        return HttpResponse::InternalServerError().await;
    }
    if let Err(err) = update_face_info_rating(
        &lose_face_info.id,
        lose_score as f64,
        false,
        req.voter.as_str(),
        now,
    )
    .await
    {
        log::error!("Error: {:?}", err);
        return HttpResponse::InternalServerError().await;
    }

    // Step 4: Add vote logs
    if let Err(err) = add_rating_logs(vec![RatingLog {
        id: resource::id_generator::get_id().await,
        win_face_id: win_face_info.id.clone(),
        loss_face_id: lose_face_info.id.clone(),
        creator: req.voter.clone(),
        created_on: now,
        ..RatingLog::default()
    }])
    .await
    {
        log::error!("Error: {:?}", err);
    }

    Ok(HttpResponse::Ok().json(()))
}

async fn check_add_face_info_param(face_info: &FaceInfo) -> Result<(), Error> {
    if face_info.id.is_empty() {
        return Err(ErrorInternalServerError("generate id failed"));
    }

    if face_info.file_id.is_empty() {
        return Err(ErrorBadRequest("file id is empty"));
    }

    if face_info.star_name.is_empty() {
        return Err(ErrorBadRequest("start name is empty"));
    }

    Ok(())
}
