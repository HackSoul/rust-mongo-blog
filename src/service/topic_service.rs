use crate::entity::topic;
use crate::dao::topic_repository;

use mongodb::Bson;
use mongodb::ordered::OrderedDocument;
use mongodb::coll::results::{UpdateResult, DeleteResult};

pub fn find_topic_list() -> Vec<OrderedDocument> {
    let cursor = topic_repository::find_topic_list();
    let mut doc_list: Vec<OrderedDocument> = vec![];
    for result in cursor {
        if let Ok(item) = result {
            doc_list.push(item);
        }
    }
    doc_list
}

pub fn create_topic(topic: topic::Topic) -> Bson {
    let result = topic_repository::create_topic(topic);
    result.inserted_id.unwrap()
}

pub fn update_topic(id: &String, name: &String) -> UpdateResult {
    topic_repository::update_topic(id, name)
}

pub fn delete_topic(id: &String) -> DeleteResult {
    topic_repository::delete_topic(id)
}