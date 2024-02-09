//cargo install sqlx-cli

use std::error::Error;
//use sqlx::Connection;
//use sqlx::Row;

struct Book{
   pub title: String,
   pub author: String,
   pub isbn: String,
}



async fn update(book: &Book, isbn: &str, pool: &sqlx::PgPool)-> Result<(), Box<dyn Error>>
{
   let query = "UPDATE book1 SET title = $1, author = $2 WHERE isbn = $3";
   sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&isbn)
        .execute(pool)
        .await?;
        Ok(())
}


// async fn read(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>>{
//    let q = "SELECT title, author, isbn FROM book1";
//    let query = sqlx::query(q);
//     let row = query.fetch_one(conn).await?;
//     let book = Book{
//       title: row.get("title"),
//       author:row.get("author"),
//       isbn:row.get("isbn"),
//     };

//    Ok(book)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let url = "postgres://postgres:sjualyoacp@localhost:5432/postgres";
   let pool = sqlx::postgres::PgPool::connect(url).await?;

   sqlx::migrate!("./migrations").run(&pool).await?;
    //create IF NOT EXISTS unique index book_isbn_idx on book (isbn);
   let book = Book{
      title: "shiva".to_string(),
      author: "Siva King".to_string(),
      isbn: "978-0-385-0075-2".to_string(),
   };
 
   update(&book, &book.isbn, &pool).await?;
   Ok(())
}





