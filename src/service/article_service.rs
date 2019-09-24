use crate::dao::article_repository;
use crate::entity::article::Article;

use mongodb::Bson;
use mongodb::ordered::OrderedDocument;
use mongodb::coll::results::{UpdateResult, DeleteResult};
use std::time::SystemTime;

pub fn find_article_list() -> Vec<OrderedDocument> {
    let cursor = article_repository::find_article_list();
    let mut doc_list: Vec<OrderedDocument> = vec![];
    for result in cursor {
        if let Ok(item) = result {
            doc_list.push(item);
        }
    }
    doc_list
}

pub fn create_article(title: &String, topic_id: &String, tags: &Vec<String>, markdown: &String) -> Bson {
    let result = article_repository::create_article(Article{
        title: String::from(title),
        topic_id: String::from(topic_id),
        create_date: SystemTime::now(),
        tags: tags.clone(),
        view_count: 0,
        markdown: String::from(markdown)
    });
    result.inserted_id.unwrap()
}

pub fn update_article(id: &String, title: &String, tags: &Vec<String>, markdown: &String) -> UpdateResult {
    article_repository::update_article(id, title, tags, markdown)
}

pub fn delete_article(id: &String) -> DeleteResult {
    article_repository::delete_article(id)
}