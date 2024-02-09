use std::error::Error;
use sqlx::Connection;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let url = "postgres://postgres:sjualyoacp@localhost:5432/postgres";
   let mut pool = sqlx::postgres::PgConnection::connect(url).await?;
   
  let res = sqlx::query("select 1+1 as sum")
             .fetch_one(& mut pool)
             .await?;
   let sum:i32 = res.get("sum");
   println!("1 + 1 = {}", sum);
   Ok(())
}


use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgres://postgres:sjualyoacp@localhost:5432/postgres", NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author1 (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book1  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ")?;

    Ok(())

}



// PGPASSWORD="sjualyoacp" psql -U postgres -h 0.0.0.0 -p 5432 postgres
// psql (16.0, server 16.1)
// Type "help" for help.

// postgres=# \dt
//               List of relations
//  Schema |       Name       | Type  |  Owner   
// --------+------------------+-------+----------
//  public | _sqlx_migrations | table | postgres
//  public | author           | table | postgres
//  public | book             | table | postgres
//  public | livedata1        | table | postgres
// (4 rows)










//cargo install sqlx-cli

//~/.cargo/bin/sqlx migrate build-script


use std::error::Error;
//use sqlx::Connection;
//use sqlx::Row;

struct Book{
   pub title: String,
   pub author: String,
   pub isbn: String,
}

async fn create(book: &Book, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>>
{
   
   let query = " INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";
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

   //sqlx::migrate!("./migrations").run(&pool).await?;
    //create IF NOT EXISTS unique index book_isbn_idx on book (isbn);
   let book = Book{
      title: "Salem's Lot".to_string(),
      author: "Stephen King".to_string(),
      isbn: "978-0-385-0075-2".to_string(),
   };
   create(&book, &pool).await?;
   Ok(())
}



