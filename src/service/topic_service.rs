use crate::entity::topic;
use crate::dao::topic_repository;
use mongodb::Bson;
use mongodb::cursor::Cursor;
use mongodb::ordered::OrderedDocument;
use mongodb::coll::results::UpdateResult;
use mongodb::coll::results::DeleteResult;

pub fn create_topic(topic: topic::Topic) -> Bson {
    let result = topic_repository::create_topic(topic);
    result.inserted_id.unwrap()
}

pub fn find_topic_list() -> Vec<OrderedDocument> {
    let cursor = topic_repository::find_topic_list();
    println!("{:?}", &cursor);
    let mut arr: Vec<OrderedDocument> = vec![];
    for result in cursor {
        if let Ok(item) = result {
            println!("item: {:?}", item);
            arr.push(item);
        }
    }
    arr
}

pub fn update_topic() -> UpdateResult {
    let result = topic_repository::update_topic();
    println!("{:?}", result);
    result
}

pub fn delete_topic() -> DeleteResult {
    topic_repository::delete_topic()
}