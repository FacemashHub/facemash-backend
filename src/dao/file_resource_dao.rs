use mongodb::bson::Document;
use mongodb::results::InsertOneResult;
use mongodb::Collection;

use crate::entity::file_resource::FileResource;
use crate::mongo;
use crate::resource::mongo::MONGO_CLIENT;

/// Adds a new file_resource to the "file_resource" collection in the database.
pub async fn add_one_file_resource(
    file_resource: &FileResource,
) -> mongodb::error::Result<InsertOneResult> {
    let collection: Collection<FileResource> = mongo::MONGO_CLIENT
        .get()
        .await
        .database(FileResource::db_name())
        .collection(FileResource::coll_name());
    collection.insert_one(file_resource, None).await
}

/// Gets the file_resource by doc filter.
pub async fn get_one_file_resource_by_doc_filter(
    doc_filter: Document,
) -> mongodb::error::Result<Option<FileResource>> {
    let collection = MONGO_CLIENT
        .get()
        .await
        .database(FileResource::db_name())
        .collection(FileResource::coll_name());
    collection.find_one(doc_filter, None).await
}
