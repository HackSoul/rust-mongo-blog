use crate::dao::article_repository;
use crate::entity::article::Article;

use mongodb::Bson;
use mongodb::ordered::OrderedDocument;
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