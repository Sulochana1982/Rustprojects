
//cargo install sqlx-cli

use std::error::Error;
use sqlx::Connection;
use sqlx::Row;

struct Book{
   pub title: String,
   pub author: String,
   pub isbn: String,
}

async fn create(book: &Book, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>>
{
   
   let query = " INSERT INTO book1 (title, author, isbn) VALUES ($1, $2, $3)";
   sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(conn)
        .await?;

       
        Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let url = "postgres://postgres:sjualyoacp@localhost:5432/postgres";
   let pool = sqlx::postgres::PgPool::connect(url).await?;

   sqlx::migrate!("./migrations").run(&pool).await?;
    //create IF NOT EXISTS unique index book_isbn_idx on book (isbn);
   let book = Book{
      title: "Salem's Lot".to_string(),
      author: "Stephen King".to_string(),
      isbn: "978-0-385-0075-2".to_string(),
   };
   create(&book, &pool).await?;
   Ok(())
}
//cargo run -q






//tokio = {version = "0.2.25", features = ["full"]}
//tokio = {version = "1.0", features = ["full"]}


//cargo install sqlx-cli
//#[allow(unused)]

use std::error::Error;
//use sqlx::Connection;
use sqlx::Row;

struct Book{
   pub title: String,
   pub author: String,
   pub isbn: String,
}

async fn create(book: &Book, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>>
{
   
   let query = " INSERT INTO book1 (title, author, isbn) VALUES ($1, $2, $3)";
   sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(conn)
        .await?;

       
        Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let url = "postgres://postgres:sjualyoacp@localhost:5432/postgres";
   let pool = sqlx::postgres::PgPool::connect(url).await?;

   sqlx::migrate!("./migrations").run(&pool).await?;
    //create IF NOT EXISTS unique index book_isbn_idx on book (isbn);
   let book = Book{
      title: "Salem's Lot".to_string(),
      author: "Stephen King".to_string(),
      isbn: "978-0-385-0075-8".to_string(),
   };
   create(&book, &pool).await?;
   Ok(())
}

//cargo run -q


























































