use crate::service::file_service::init_local_directory;

pub mod face_info_service;
pub mod file_service;

pub async fn init_file_service() {
    init_local_directory().await;
}
