extern crate chrono;

#[macro_use]
extern crate diesel;

extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let chapter = models::NewChapter {
        title: String::from("Swann Herrera"),
        author: String::from("Le début de l'énigme"),
        published: true,
    };

    if models::Chapters::insert(chapter, &conn) {
        println!("success");
    } else {
        println!("fail");
    }
}
