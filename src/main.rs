#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::prelude::*;

mod models;
mod schema;

async fn index(param: web::Path<i32>) -> impl Responder {
    println!("index");
    let id: i32 = *param;
    let conn = get_connection();
    let chapters = models::Chapters::show(id, &conn);
    let chapter = chapters[0].clone();
 
    web::Json(chapter)
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/chapter/{id}/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind(get_url())?
    .run()
    .await
}

fn get_url() -> String {
    dotenv().ok();
    let host = env::var("HOST").expect("set HOST");
    let port = env::var("PORT").expect("set PORT");
    format!("{}:{}", host, port)
}

fn get_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    return PgConnection::establish(&database_url).unwrap()
}