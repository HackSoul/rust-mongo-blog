use crate::api::request::article_create_request::ArticleCreateRequest;
use crate::api::request::article_update_request::ArticleUpdateRequest;
use crate::api::request::article_delete_request::ArticleDeleteRequest;
use crate::api::response::article_update_response::ArticleUpdateResponse;
use crate::api::response::article_delete_response::ArticleDeleteResponse;
use crate::service::article_service;

use actix_web::{web, Result};
use mongodb::Bson;
use mongodb::ordered::OrderedDocument;
use crate::entity::article::Article;
use std::time::SystemTime;

pub fn find_article_list() -> Result<web::Json<Vec<OrderedDocument>>> {
    let result = article_service::find_article_list();
    Ok(web::Json(result))
}

pub fn create_article(info: web::Json<ArticleCreateRequest>) -> Result<web::Json<Bson>> {
    let bson = article_service::create_article(Article{
        title: String::from(&info.title),
        category: String::from(&info.category),
        technology: String::from(&info.technology),
        create_date: SystemTime::now(),
        tags: (&info.tags).clone(),
        view_count: 0,
        introduce: String::from(&info.introduce),
    });
    Ok(web::Json(bson))
}

pub fn update_article(info: web::Json<ArticleUpdateRequest>) -> Result<web::Json<ArticleUpdateResponse>>  {
    let result = article_service::update_article(&info.id, &info.title, &info.category, &info.technology, &info.tags, &info.introduce);
    let resp = ArticleUpdateResponse {modified_count: result.modified_count};
    Ok(web::Json(resp))
}

pub fn delete_article(info: web::Json<ArticleDeleteRequest>) -> Result<web::Json<ArticleDeleteResponse>> {
    let result = article_service::delete_article(&info.id);
    let resp = ArticleDeleteResponse {deleted_count: result.deleted_count};
    Ok(web::Json(resp))
}

