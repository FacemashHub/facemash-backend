use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UriType {
    Local,
    Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FileResource {
    pub id: String,
    pub file_name: String,
    pub file_uri: String,
    pub uri_type: UriType,
    pub md5: String,
    pub thumb_uri: String,
    pub thumb_type: UriType,
    pub creator: String,
    pub updater: String,
    pub created_on: i64,
    pub updated_on: i64,
    pub deleted_on: i64,
    pub is_deleted: i64,
}

impl Default for FileResource {
    fn default() -> Self {
        FileResource {
            id: "".to_string(),
            file_name: "".to_string(),
            file_uri: "".to_string(),
            uri_type: UriType::Local,
            md5: "".to_string(),
            thumb_uri: "".to_string(),
            thumb_type: UriType::Local,
            creator: "".to_string(),
            updater: "".to_string(),
            created_on: 0,
            updated_on: 0,
            deleted_on: 0,
            is_deleted: 0,
        }
    }
}

impl FileResource {
    pub fn db_name() -> &'static str {
        "facemash"
    }

    pub fn coll_name() -> &'static str {
        "file_resource"
    }
}
