use actix_web::{web, Result};
use serde_derive::{Serialize, Deserialize};
use crate::service::topic_service;
use crate::entity::topic::Topic;
use std::time::SystemTime;
use mongodb::Bson;
use mongodb::cursor::Cursor;
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
    OK("haha")
}