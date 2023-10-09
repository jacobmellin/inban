use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

pub mod models;
pub mod schema;

use models::*;
use schema::books::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)) 
}

pub fn get_books() -> Vec<Book> {
    let mut connection = establish_connection();
    books.limit(5)
         .load::<Book>(&mut connection)
         .expect("Error loading books")
}

pub fn add_book(book_name: &str) -> Result<usize, String> {
    let mut connection = establish_connection();
    let uuid = Uuid::new_v4().to_string();

    let new_book = NewBook{ id:&uuid, name: book_name };

    let result = diesel::insert_into(books)
        .values(&new_book)
        .execute(&mut connection);

    match result {
        Ok(result) => return Ok(result),
        Err(e) => Err(format!("Could not create book: {}", e))
    }
}
