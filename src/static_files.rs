#![feature(proc_macro_hygiene, decl_macro)]

use rocket::response::NamedFile;
use std::io;
use std::path::Path;

#[get("/")]
fn index() -> io::Result<NamedFile> {
  NamedFile::open("public/index.html")
}

#[get("/<file..>", rank = 5)]
fn all() -> Option<NamedFile> {
  NamedFile::open(Path::new("public/").join(file)).ok()
}
