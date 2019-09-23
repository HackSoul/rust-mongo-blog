use crate::dao::mongo_connector;
use mongodb::ThreadedClient;
use mongodb::{Bson, bson, doc};
use mongodb::db::ThreadedDatabase;
use crate::entity::topic::Topic;
use mongodb::cursor::Cursor;

use chrono::offset::Utc;
use chrono::DateTime;
use mongodb::coll::results::InsertOneResult;

pub fn create_topic(topic: Topic) -> InsertOneResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("topic");

    let datetime: DateTime<Utc> = topic.create_date.into();

    let doc = doc! {
        "name": topic.name,
        "create_date": format!("{}", datetime.format("%Y-%m-%d %T")),
    };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.")
}

pub fn find_topic_list() -> Cursor {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("topic");
    coll.find(None, None).unwrap()
}