use std::env;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::Client;

use crate::config::MONGODB_URI;

lazy_static! {
    pub static ref MONGO_CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        let uri = env::var(MONGODB_URI).expect("You must set the MONGODB_URI environment var!");
        Client::with_uri_str(&uri).await.unwrap()
    });
}
