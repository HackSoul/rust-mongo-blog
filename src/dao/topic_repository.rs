use crate::dao::mongo_connector;
use crate::entity::topic::Topic;

use mongodb::{Bson, bson, doc, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::results::{InsertOneResult, UpdateResult, DeleteResult};
use mongodb::cursor::Cursor;
use mongodb::oid::ObjectId;

use chrono::offset::Utc;
use chrono::DateTime;

pub fn create_topic(topic: Topic) -> InsertOneResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("topic");

    let datetime: DateTime<Utc> = topic.create_date.into();

    let doc = doc! {
        "name": topic.name,
        "create_date": format!("{}", datetime.format("%Y-%m-%d %T")),
    };

    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.")
}

pub fn find_topic_list() -> Cursor {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("topic");
    coll.find(None, None).unwrap()
}

pub fn update_topic() -> UpdateResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("topic");
    let id = Bson::ObjectId(ObjectId::with_string("5d888d78613465ee27188d6b").unwrap());
    coll.update_one(doc!{"_id": id}, doc!{ "$set": {"name": "xujiyou123"} }, None).unwrap()
}

pub fn delete_topic() -> DeleteResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("topic");
    let id = Bson::ObjectId(ObjectId::with_string("5d888d78613465ee27188d6b").unwrap());
    coll.delete_many(doc!{"_id": id}, None).unwrap()
}