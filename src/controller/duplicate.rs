use actix_web::{post, web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DuplicateReq {
    name: String,
    duplicate_time: isize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DuplicateResp {
    res: String,
}

#[post("/duplicate")]
pub async fn duplicate(req: web::Json<DuplicateReq>) -> Result<impl Responder, Error> {
    log::debug!("req: {:?}", &req);

    Ok(HttpResponse::Ok().json(DuplicateResp {
        res: req.name.repeat(req.duplicate_time as usize),
    }))
}
