use mongodb::{Client, ThreadedClient};

pub fn get_conn() -> Client{
    Client::with_uri("mongodb://xujiyou:password@localhost:27017")
        .expect("Failed to initialize standalone client.")
}