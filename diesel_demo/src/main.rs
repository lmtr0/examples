
use diesel::{prelude::*};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::io::{stdin, Read};

use crate::models::Post;
mod schema;
mod models;

#[macro_use]
extern crate diesel;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}




fn main() {
    println!("Hello world!");

    let connection = establish_connection();
    
    
    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    models::create_post(&connection, title, &body);
    println!("\nSaved draft {}", title);
    #[cfg(not(windows))]
    const EOF: &'static str = "CTRL+D";
    
    #[cfg(windows)]
    const EOF: &'static str = "CTRL+Z";
    
    use crate::schema::posts::dsl::{posts};
    
    // get all
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}