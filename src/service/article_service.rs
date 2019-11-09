use crate::dao::article_repository;
use crate::entity::article::Article;

use mongodb::Bson;
use mongodb::ordered::OrderedDocument;
use mongodb::coll::results::{UpdateResult, DeleteResult};

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

pub fn create_article(article: Article) -> Bson {
    let result = article_repository::create_article(article);
    result.inserted_id.unwrap()
}

pub fn update_article(id: &String, title: &String, category: &String, technology: &String, tags: &Vec<String>) -> UpdateResult {
    article_repository::update_article(id, title, category, technology, tags)
}

pub fn delete_article(id: &String) -> DeleteResult {
    article_repository::delete_article(id)
}