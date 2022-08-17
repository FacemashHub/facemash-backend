use std::fs::File;
use std::io::{Error, Read};

use crypto::digest::Digest;
use crypto::md5::Md5;

pub async fn get_file_md5(filepath: &str) -> Result<String, Error> {
    let mut f = File::open(filepath)?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    let mut hasher = Md5::new();
    hasher.input(&buffer);

    Ok(hasher.result_str())
}

#[actix_rt::test]
async fn test_get_file_md5() {
    println!("{}", get_file_md5("./README.md").await.unwrap())
}
