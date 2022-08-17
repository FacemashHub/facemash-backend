use mongodb::bson::Document;
use mongodb::results::InsertOneResult;

use crate::dao::face_info_dao;
use crate::entity::face_info::FaceInfo;

pub async fn get_face_info_randomly(size: i64) -> Result<Vec<FaceInfo>, mongodb::error::Error> {
    face_info_dao::get_face_info_sample(size).await
}

pub async fn get_one_face_info_by_doc_filter(
    doc_filter: Document,
) -> mongodb::error::Result<Option<FaceInfo>> {
    face_info_dao::get_one_face_info_by_doc_filter(doc_filter).await
}

pub async fn add_face_info(face_info: &FaceInfo) -> mongodb::error::Result<InsertOneResult> {
    face_info_dao::add_face_info(face_info).await
}
