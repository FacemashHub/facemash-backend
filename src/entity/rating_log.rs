use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RatingLog {
    pub id: String,
    pub face_id: String,
    pub vote_behavior: i32, // -1: LOSS, 0: DRAW, 1: WIN
    pub creator: String,
    pub created_on: i64,
    pub deleted_on: i64,
    pub is_deleted: i64,
}

impl Default for RatingLog {
    fn default() -> Self {
        RatingLog {
            id: "".to_string(),
            face_id: "".to_string(),
            vote_behavior: 0,
            creator: "".to_string(),
            created_on: 0,
            deleted_on: 0,
            is_deleted: 0,
        }
    }
}

impl RatingLog {
    pub fn db_name() -> &'static str {
        "facemash"
    }

    pub fn coll_name() -> &'static str {
        "rating_log"
    }
}
