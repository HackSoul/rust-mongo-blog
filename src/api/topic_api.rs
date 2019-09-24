use crate::service::topic_service;
use crate::entity::topic::Topic;

use actix_web::{web, Result};
use serde_derive::Deserialize;

use std::time::SystemTime;
use mongodb::Bson;
use mongodb::ordered::OrderedDocument;

#[derive(Debug, Deserialize)]
pub struct Info {
    name: String,
}

pub fn create_topic(info: web::Query<Info>) -> Result<web::Json<Bson>> {
    let bson = topic_service::create_topic(Topic{
        name: String::from(&info.name),
        create_date: SystemTime::now()
    });
    println!("{:?}", &bson);
    Ok(web::Json(bson))
}

pub fn find_topic_list() -> Result<web::Json<Vec<OrderedDocument>>>{
    let result = topic_service::find_topic_list();
    Ok(web::Json(result))
}

pub fn update_topic() -> Result<String> {
    topic_service::update_topic();
    Ok(String::from("hah"))
}

pub fn delete_topic() -> Result<String> {
    let result = topic_service::delete_topic();
    println!("result is: {:?}", result);
    Ok(String::from("haha"))
}