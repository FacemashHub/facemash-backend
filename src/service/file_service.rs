use std::fs;
use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{error, web, Error};
use futures_util::TryStreamExt as _;

pub static SAVE_DIR: &str = "./tmp";

pub async fn init_local_directory() {
    fs::create_dir_all(SAVE_DIR).unwrap()
}

pub async fn save_file_in_payload(
    mut payload: Multipart,
    face_info_id: &str,
) -> Result<String, Error> {
    let mut filename: String = "".to_string();

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        filename = match content_disposition.get_filename() {
            None => {
                return Err(error::ErrorInternalServerError(
                    "Couldn't read the filename.",
                ));
            }
            Some(f_name) => {
                info!("{}", f_name);
                f_name.replace(' ', "_").to_string()
            }
        };

        // .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
        let filepath = get_local_filepath(face_info_id, &filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(filename)
}

pub async fn delete_file(filepath: &str) {
    match fs::remove_file(filepath) {
        Ok(_) => {}
        Err(err) => {
            error!("Failed to remove file: {:?}, error: {:?}", filepath, err)
        }
    };
}

pub fn get_local_filepath(face_info_id: &str, filename: &str) -> String {
    format!("{SAVE_DIR}/{face_info_id}-{filename}")
}
