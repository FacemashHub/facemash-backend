#![feature(once_cell)]
#[macro_use]
extern crate log;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use mongodb::bson::doc;

use crate::controller::{face_info_controller, file_controller};
use crate::resource::mongo;

mod config;
mod controller;
mod dao;
mod entity;
mod logger;
mod resource;
mod service;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logger::init();

    resource::check_resources().await;
    service::init_file_service().await;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(face_info_controller::get_face_info_randomly)
            .service(face_info_controller::get_face_info_by_id)
            .service(face_info_controller::add_face_info)
            .service(file_controller::save_file)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
