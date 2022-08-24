use futures_util::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::results::{InsertOneResult, UpdateResult};
use mongodb::{bson, Collection};

use crate::entity::face_info::FaceInfo;
use crate::mongo;
use crate::resource::mongo::MONGO_CLIENT;

/// Adds a new face_info to the "face_info" collection in the database.
pub async fn add_one_face_info(face_info: &FaceInfo) -> mongodb::error::Result<InsertOneResult> {
    let collection: Collection<FaceInfo> = mongo::MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());
    collection.insert_one(face_info, None).await
}

/// Gets the face_info by doc filter.
pub async fn get_one_face_info_by_doc_filter(
    doc_filter: Document,
) -> mongodb::error::Result<Option<FaceInfo>> {
    let collection = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());
    collection.find_one(doc_filter, None).await
}

/// Update the face_info by id.
pub async fn update_face_info_by_doc_filter(
    doc_filter: Document,
    face_info: &FaceInfo,
) -> mongodb::error::Result<UpdateResult> {
    let collection: Collection<FaceInfo> = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());

    let update = doc! {"$set": bson::to_bson(face_info).unwrap()};

    collection.update_one(doc_filter, update, None).await
}

/// Get face_info randomly
pub async fn get_face_info_sample(size: i64) -> Result<Vec<FaceInfo>, mongodb::error::Error> {
    let collection: Collection<FaceInfo> = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());

    let pipeline = vec![doc! {"$sample": {"size": size}}];

    let mut ret_face_infos: Vec<FaceInfo> = Vec::new();
    let mut results = collection.aggregate(pipeline, None).await?;
    while let Some(result) = results.next().await {
        // Use serde to deserialize into the MovieSummary struct:
        let face_info: FaceInfo = bson::from_document(result?)?;
        ret_face_infos.push(face_info);
    }
    Ok(ret_face_infos)
}
