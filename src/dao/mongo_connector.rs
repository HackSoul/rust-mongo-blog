use mongodb::{Client, ThreadedClient};

pub fn get_conn() -> Client{
    Client::with_uri("mongodb://xujiyou:password@118.126.82.184:28018")
        .expect("Failed to initialize standalone client.")
}