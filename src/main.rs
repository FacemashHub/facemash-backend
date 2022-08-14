#[macro_use]
extern crate log;

use actix_web::{middleware, web, App, HttpServer};

use crate::controller::duplicate::duplicate;
use crate::controller::upload_files::{index, save_file};

mod controller;
mod dao;
mod entity;
mod logger;
mod service;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    // std::env::set_var("RUST_LOG", "info");
    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(duplicate)
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
