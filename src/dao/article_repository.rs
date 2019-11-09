use crate::dao::mongo_connector;
use crate::entity::article::Article;

use mongodb::coll::results::InsertOneResult;
use mongodb::coll::results::UpdateResult;
use mongodb::coll::results::DeleteResult;
use mongodb::{Bson, bson, doc, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::cursor::Cursor;
use mongodb::oid::ObjectId;

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
        "category": article.category,
        "technology": article.technology,
        "create_date": format!("{}", datetime.format("%Y-%m-%d %T")),
        "tags": Bson::Array(tags_list),
        "view_count": article.view_count
    };

    coll.insert_one(doc, None)
        .ok().expect("Failed to insert document.")
}

pub fn update_article(id: &String, title: &String, category: &String, technology: &String, tags: &Vec<String>) -> UpdateResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");

    let id = Bson::ObjectId(ObjectId::with_string(id).unwrap());
    let mut tags_list:Vec<Bson> = vec![];
    for str in tags {
        tags_list.push(Bson::String(String::from(str)));
    }
    let doc = doc! {
        "$set": {
            "title": title,
            "category": category,
            "technology": technology,
            "tags": tags_list
        }
    };

    coll.update_one(doc!{"_id": id}, doc, None).unwrap()
}

pub fn delete_article(id: &String) -> DeleteResult {
    let client = mongo_connector::get_conn();
    let coll = client.db("blog").collection("article");
    let id = Bson::ObjectId(ObjectId::with_string(id).unwrap());
    coll.delete_many(doc!{"_id": id}, None).unwrap()
}