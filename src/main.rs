#[macro_use]
extern crate log;

use actix_web::{post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod logger;

#[derive(Debug, Serialize, Deserialize)]
struct DuplicateReq {
    name: String,
    duplicate_time: isize,
}

#[derive(Debug, Serialize, Deserialize)]
struct DuplicateResp {
    res: String,
}

#[post("/duplicate")]
async fn duplicate(req: web::Json<DuplicateReq>) -> Result<impl Responder, Error> {
    log::debug!("req: {:?}", &req);

    Ok(HttpResponse::Ok().json(DuplicateResp {
        res: req.name.repeat(req.duplicate_time as usize),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    HttpServer::new(|| App::new().service(duplicate))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
