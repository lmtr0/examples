use diesel::{Insertable, SqliteConnection};

use super::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}



pub fn create_post<'a>(conn: &SqliteConnection, _title: &'a str, _body: &'a str) {
    let new_post = NewPost {
        title: _title,
        body: _body,
    };

    use diesel::RunQueryDsl;

    diesel::insert_into(crate::schema::posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}