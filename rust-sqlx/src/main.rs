// use sqlx::

use chrono::Utc;
use serde_json::{json, Map};
// use sqlx::query::Map;
use sqlx::{postgres::PgPool, FromRow};

#[derive(Debug, Clone, FromRow)]
struct Book {
    pub id: uuid::Uuid,
    pub title: String,
    pub publish_date: chrono::DateTime<Utc>,
    pub meta: serde_json::Map<String, serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let _ = dotenv::dotenv();
    println!("Hello, world!");

    let uuid = uuid::Uuid::new_v4();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").expect("Failed to get database_url"))
        .await?;

    let book = Book {
        id: uuid::Uuid::new_v4(),
        title: "A new book".to_string(),
        publish_date: Utc::now(),
        meta: Map::from_iter([("Hello".to_string(), 60.into())]),
    };

    // sqlx::query("")

    let start = std::time::Instant::now();

    let res = sqlx::query!(
        "insert into testdb.books values ($1, $2, $3, $4)",
        book.id,
        book.title,
        book.publish_date,
        json!(book.meta)
    ).execute(&pool).await?;

    println!("{res:#?} in {:?}", start.elapsed());

    // let result = sqlx::query_as!(Record, "insert into testdb.books values (?, ?, ?, ?);");

    Ok(())
    // sqlx::query_as!()
}
