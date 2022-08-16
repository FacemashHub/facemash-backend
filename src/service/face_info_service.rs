use crate::dao::face_info_dao;
use crate::entity::face_info::FaceInfo;
use mongodb::results::InsertOneResult;

pub async fn get_face_info_randomly(size: i64) -> Result<Vec<FaceInfo>, mongodb::error::Error> {
    face_info_dao::get_face_info_sample(size).await
}

pub async fn get_face_info_by_id(id: &str) -> mongodb::error::Result<Option<FaceInfo>> {
    face_info_dao::get_face_info_by_id(id).await
}

pub async fn add_face_info(face_info: &FaceInfo) -> mongodb::error::Result<InsertOneResult> {
    face_info_dao::add_face_info(face_info).await
}
