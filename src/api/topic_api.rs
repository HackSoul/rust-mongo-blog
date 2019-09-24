use crate::service::topic_service;
use crate::entity::topic::Topic;
use crate::api::request::topic_create_request::TopicCreateRequest;
use crate::api::request::topic_update_request::TopicUpdateRequest;
use crate::api::request::topic_delete_request::TopicDeleteRequest;
use crate::api::response::topic_update_response::TopicUpdateResponse;
use crate::api::response::topic_delete_response::TopicDeleteResponse;

use actix_web::{web, Result};

use std::time::SystemTime;
use mongodb::Bson;
use mongodb::ordered::OrderedDocument;

pub fn find_topic_list() -> Result<web::Json<Vec<OrderedDocument>>>{
    let result = topic_service::find_topic_list();
    Ok(web::Json(result))
}

pub fn create_topic(info: web::Json<TopicCreateRequest>) -> Result<web::Json<Bson>> {
    let bson = topic_service::create_topic(Topic{
        name: String::from(&info.name),
        create_date: SystemTime::now()
    });
    Ok(web::Json(bson))
}

pub fn update_topic(info: web::Json<TopicUpdateRequest>) -> Result<web::Json<TopicUpdateResponse>> {
    let result = topic_service::update_topic(&info.id, &info.name);
    let resp = TopicUpdateResponse{modified_count: result.modified_count};
    Ok(web::Json(resp))
}

pub fn delete_topic(info: web::Json<TopicDeleteRequest>) -> Result<web::Json<TopicDeleteResponse>> {
    let result = topic_service::delete_topic(&info.id);
    let resp = TopicDeleteResponse{deleted_count: result.deleted_count};
    Ok(web::Json(resp))
}