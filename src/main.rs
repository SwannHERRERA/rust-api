#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::etablish(&database_url).unwrap();

    let chapter = model::NewChapter {
        title: String::from("Swann Herrera"),
        author: String::from("Le début de l'énigme"),
        published: true,
    };

    if models::Chapter::insert(chapter, &conn) {
        println!("success");
    } else {
        println!("fail");
    }

    println!("{}", String::from("Hello, World !"));
}
