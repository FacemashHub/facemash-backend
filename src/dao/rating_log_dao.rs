use mongodb::results::InsertManyResult;
use mongodb::Collection;

use crate::entity::rating_log::RatingLog;
use crate::mongo;

/// Adds new rating_logs to the "rating_log" collection in the database.
pub async fn add_rating_logs(
    rating_log: Vec<RatingLog>,
) -> mongodb::error::Result<InsertManyResult> {
    let collection: Collection<RatingLog> = mongo::MONGO_CLIENT
        .get()
        .await
        .database(RatingLog::db_name())
        .collection(RatingLog::coll_name());
    collection.insert_many(rating_log, None).await
}
