use crate::dao::article_repository;
use crate::entity::article::Article;

use mongodb::Bson;
use std::time::SystemTime;

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