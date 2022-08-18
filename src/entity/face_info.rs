use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UriType {
    Local,
    // Cos,
    // Oss,
    // S3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FaceInfo {
    pub id: String,
    pub file_name: String,
    pub file_uri: String,
    pub uri_type: UriType,
    pub md5: String,
    pub thumb_uri: String,
    pub thumb_type: UriType,
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
            file_name: "".to_string(),
            file_uri: "".to_string(),
            uri_type: UriType::Local,
            md5: "".to_string(),
            thumb_uri: "".to_string(),
            thumb_type: UriType::Local,
            upvote_count: 0,
            downvote_count: 0,
            score: 0.0,
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
