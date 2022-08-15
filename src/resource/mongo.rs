use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mongodb::Client;

lazy_static! {
    pub static ref MONGO_CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        let uri = "<mongodb://localhost:27017>";
        Client::with_uri_str(&uri).await.unwrap()
    });
}
