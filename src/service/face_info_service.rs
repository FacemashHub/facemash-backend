use mongodb::bson::Document;
use mongodb::results::{InsertOneResult, UpdateResult};

use crate::dao::face_info_dao;
use crate::doc;
use crate::entity::face_info::FaceInfo;

pub async fn get_face_info_randomly(size: i64) -> Result<Vec<FaceInfo>, mongodb::error::Error> {
    face_info_dao::get_face_info_sample(size).await
}

pub async fn get_one_face_info_by_doc_filter(
    doc_filter: Document,
) -> mongodb::error::Result<Option<FaceInfo>> {
    face_info_dao::get_one_face_info_by_doc_filter(doc_filter).await
}

pub async fn get_face_infos_by_doc_filter(
    doc_filter: Document,
) -> Result<Vec<FaceInfo>, mongodb::error::Error> {
    face_info_dao::get_face_infos_by_doc_filter(doc_filter).await
}

pub async fn add_face_info(face_info: &FaceInfo) -> mongodb::error::Result<InsertOneResult> {
    face_info_dao::add_one_face_info(face_info).await
}

pub async fn update_face_info_rating(
    face_info_id: &str,
    rating: i32,
    upvote: bool,
) -> mongodb::error::Result<UpdateResult> {
    let filter_doc = doc! {"id": face_info_id};

    let update_doc = if upvote {
        doc! {
            "$set": {"rating": rating},
            "$inc": {"upvote_count": 1},
        }
    } else {
        doc! {
            "$set": {"rating": rating},
            "$inc": {"downvote_count": 1},
        }
    };

    face_info_dao::update_face_info_by_doc_filter(filter_doc, update_doc).await
}
