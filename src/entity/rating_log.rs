use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RatingLog {
    pub id: String,
    pub win_face_id: String,
    pub loss_face_id: String,
    pub creator: String,
    pub updater: String,
    pub created_on: i64,
    pub updated_on: i64,
    pub deleted_on: i64,
    pub is_deleted: i64,
}

impl Default for RatingLog {
    fn default() -> Self {
        RatingLog {
            id: "".to_string(),
            win_face_id: "".to_string(),
            loss_face_id: "".to_string(),
            creator: "".to_string(),
            updater: "".to_string(),
            created_on: 0,
            updated_on: 0,
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
