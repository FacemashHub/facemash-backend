#![feature(once_cell)]
#[macro_use]
extern crate log;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use mongodb::bson::doc;

use crate::controller::face_info_controller;
use crate::controller::upload_files::{index, save_file};
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
    logger::init();
    dotenv().ok();

    resource::check_resources().await;

    // std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(face_info_controller::get_face_info_randomly)
            .service(face_info_controller::get_face_info_by_id)
            .service(face_info_controller::add_face_info)
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
