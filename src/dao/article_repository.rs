use crate::dao::mongo_connector;
use crate::entity::article::Article;

use mongodb::coll::results::InsertOneResult;
use mongodb::{Bson, bson, doc, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::cursor::Cursor;

use chrono::offset::Utc;
use chrono::DateTime;

pub fn find_article_list() -> Cursor {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");
    coll.find(None, None).unwrap()
}

pub fn create_article(article: Article) -> InsertOneResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");

    let datetime: DateTime<Utc> = article.create_date.into();
    let mut tags_list:Vec<Bson> = vec![];
    for str in article.tags {
        tags_list.push(Bson::String(str));
    }

    let doc = doc! {
        "title": article.title,
        "topic_id": article.topic_id,
        "create_date": format!("{}", datetime.format("%Y-%m-%d %T")),
        "tags": Bson::Array(tags_list),
        "view_count": article.view_count,
        "markdown": article.markdown
    };

    coll.insert_one(doc, None)
        .ok().expect("Failed to insert document.")
}