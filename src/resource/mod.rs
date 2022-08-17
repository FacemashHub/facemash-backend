use crate::doc;

pub mod id_generator;
pub mod mongo;

pub async fn check_resources() {
    check_mongo().await;
    check_id_generator().await;
}

async fn check_mongo() {
    mongo::MONGO_CLIENT
        .get()
        .await
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
        .unwrap();
    info!("Mongo connected successfully.");
}

async fn check_id_generator() {
    info!("Id generate success: {}.", id_generator::get_id().await)
}
