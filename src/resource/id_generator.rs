use std::env;
use std::sync::Mutex;

use lazy_static::lazy_static;
use snowflake::SnowflakeIdBucket;

use crate::config;

lazy_static! {
    static ref ID_GENERATOR_BUCKET: Mutex<SnowflakeIdBucket> = Mutex::new({
        let machine_id: i32 = env::var(config::SNOWFLAKE_MACHINE_ID)
            .expect("You must set the SNOWFLAKE_MACHINE_ID environment var!")
            .parse::<i32>()
            .unwrap();
        let node_id: i32 = env::var(config::SNOWFLAKE_NODE_ID)
            .expect("You must set the SNOWFLAKE_NODE_ID environment var!")
            .parse::<i32>()
            .unwrap();

        SnowflakeIdBucket::new(machine_id, node_id)
    });
}

pub async fn get_id() -> String {
    ID_GENERATOR_BUCKET.lock().unwrap().get_id().to_string()
}

#[actix_rt::test]
async fn generate_id_test() {
    use dotenv::dotenv;

    dotenv().ok();
    println!("{}", get_id().await)
}
