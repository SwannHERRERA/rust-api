use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::chapters;
use schema::chapters::dsl::chapters as all_chapters;

#[derive(Queryable)]
pub struct Chapters {
  pub id: i32,
  pub title: String,
  pub author: String,
  pub created_at: Instant,
  pub updated_at: Instant,
  pub published_at: Instant,
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
  pub fn show(id: i32, conn: &PgConnection) -> Vec<Chapter> {
    all_chapters
      .find(id)
      .load::<Chapter>(conn)
      .expect("Error loading chapter")
  }

  pub fn all(conn: &PgConnection) -> Vec<Chapter> {
    all_chapters
      .order(chapter::id.desc())
      .load::<Chapter>(conn)
      .expect("Error loading chapters")
  }

  pub fn update_by_id(id: i32, conn: &PgConnection, chapter: NewChapter) -> bool {
    use schema::chapters::dsl::{author as a, published as p, title as t};
    let NewChapter {
      title,
      author,
      published,
    } = chapter;

    diesel::update(all_chapters.find(id))
      .set((a.eq(author), p.eq(published), t.eq(title)))
      .get_result::<Chapter>(conn)
      .is_ok()
  }

  pub fn insert(chapter: NewChapter, conn: &PgConnection) -> bool {
    diesel::insert_into(chapter::table)
      .values(&chapter)
      .execute(conn)
      .is_ok()
  }

  pub fn delete_by_id(id: i32, conn: &PgConnection) -> bool {
    if Chapter::show(id, conn).is_empty() {
      return false;
    }
    diesel::delete(all_chapters.find(id)).execute(conn).is_ok()
  }

  pub fn all_by_athor(author: String, conn: &PgConnection) -> Vec<Chapter> {
    all_chapters
      .filter(chapter::author.eq(author))
      .load<Chapter>(conn)
      .expect("Error loading Chapters by author")
  }
}
