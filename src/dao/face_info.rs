use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

use crate::entity::face_info::FaceInfo;
use crate::resource::mongo::MONGO_CLIENT;

/// Adds a new face_info to the "face_info" collection in the database.
pub async fn add_face_info(face_info: FaceInfo) -> mongodb::error::Result<InsertOneResult> {
    let collection = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());
    collection.insert_one(face_info, None).await
}

/// Gets the face_info by id.
pub async fn get_face_info(id: String) -> mongodb::error::Result<Option<FaceInfo>> {
    let collection = MONGO_CLIENT
        .get()
        .await
        .database(FaceInfo::db_name())
        .collection(FaceInfo::coll_name());
    collection.find_one(doc! { "id": &id }, None).await
}
