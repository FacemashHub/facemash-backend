use mongodb::results::InsertOneResult;
use mongodb::Collection;

use crate::entity::rating_log::RatingLog;
use crate::mongo;

/// Adds a new rating_log to the "rating_log" collection in the database.
pub async fn add_one_rating_log(rating_log: &RatingLog) -> mongodb::error::Result<InsertOneResult> {
    let collection: Collection<RatingLog> = mongo::MONGO_CLIENT
        .get()
        .await
        .database(RatingLog::db_name())
        .collection(RatingLog::coll_name());
    collection.insert_one(rating_log, None).await
}
