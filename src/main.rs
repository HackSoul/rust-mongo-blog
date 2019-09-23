mod api;
mod service;
mod dao;
mod entity;

extern crate mongodb;
extern crate chrono;

use actix_web::{middleware, web, App, HttpServer};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(api::topic_api::create_topic))
            .service(web::resource("/list").to(api::topic_api::find_topic_list))
    }).bind("127.0.0.1:8080")?.run()
}