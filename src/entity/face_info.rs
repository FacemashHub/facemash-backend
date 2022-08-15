use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceInfo {
    pub id: String,
    pub file_name: String,
    pub file_path: String,
    pub upvote_count: u64,
    pub downvote_count: u64,
    pub score: f64,
    pub creator: String,
    pub updater: String,
    pub created_on: i64,
    pub updated_on: i64,
    pub deleted_on: i64,
    pub is_deleted: i64,
}

impl FaceInfo {
    pub fn db_name() -> &'static str {
        "facemash"
    }

    pub fn coll_name() -> &'static str {
        "face_info"
    }
}
