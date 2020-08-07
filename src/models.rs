use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use chrono::NaiveDateTime;

use crate::schema::chapters;
use crate::schema::chapters::dsl::chapters as all_chapters;

#[derive(Queryable)]
pub struct Chapters {
  pub id: i32,
  pub title: String,
  pub author: String,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
  pub published_at: Option<NaiveDateTime>,
  pub published: bool,
}

#[derive(Insertable)]
#[table_name = "chapters"]
pub struct NewChapter {
  pub title: String,
  pub author: String,
  pub published: bool,
}

impl Chapters {
  pub fn show(id: i32, conn: &PgConnection) -> Vec<Chapters> {
    all_chapters
      .find(id)
      .load::<Chapters>(conn)
      .expect("Error loading chapter")
  }

  pub fn all(conn: &PgConnection) -> Vec<Chapters> {
    all_chapters
      .order(chapters::id.desc())
      .load::<Chapters>(conn)
      .expect("Error loading chapters")
  }

  pub fn update_by_id(id: i32, conn: &PgConnection, chapters: NewChapter) -> bool {
    use crate::schema::chapters::dsl::{author as a, published as p, title as t};
    let NewChapter {
      title,
      author,
      published,
    } = chapters;

    diesel::update(all_chapters.find(id))
      .set((a.eq(author), p.eq(published), t.eq(title)))
      .get_result::<Chapters>(conn)
      .is_ok()
  }

  pub fn insert(chapter: NewChapter, conn: &PgConnection) -> bool {
    diesel::insert_into(chapters::table)
      .values(&chapter)
      .execute(conn)
      .is_ok()
  }

  pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
    if Chapters::show(id, conn).is_empty() {
      return false;
    }
    diesel::delete(all_chapters.find(id)).execute(conn).is_ok()
  }

  pub fn all_by_athor(author: String, conn: &PgConnection) -> Vec<Chapters> {
    all_chapters
      .filter(chapters::author.eq(author))
      .load::<Chapters>(conn)
      .expect("Error loading Chapters by author")
  }
}
