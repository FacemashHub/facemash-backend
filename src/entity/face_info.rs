use serde::{Deserialize, Serialize};

pub const DEFAULT_SCORE: f64 = 1400.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FaceInfo {
    pub id: String,
    pub star_name: String,
    pub file_id: String,
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

impl Default for FaceInfo {
    fn default() -> Self {
        FaceInfo {
            id: "".to_string(),
            file_id: "".to_string(),
            star_name: "".to_string(),
            upvote_count: 0,
            downvote_count: 0,
            score: DEFAULT_SCORE,
            creator: "".to_string(),
            updater: "".to_string(),
            created_on: 0,
            updated_on: 0,
            deleted_on: 0,
            is_deleted: 0,
        }
    }
}

impl FaceInfo {
    pub fn db_name() -> &'static str {
        "facemash"
    }

    pub fn coll_name() -> &'static str {
        "face_info"
    }
}
