
//cargo install sqlx-cli

use std::error::Error;
//use sqlx::Connection;
//use sqlx::Row;

use futures::{ future, StreamExt };
use yahoo_finance::Streamer;

struct DataSample{
   pub timestamp: i64,
   pub symbol: String,
   pub price: f64,
}

async fn create(book: &DataSample, conn: &sqlx::PgPool) 
{
   
   let query = "INSERT INTO livedata1 (title, author, isbn) VALUES ($1, $2, $3)";
   sqlx::query(query)
        .bind(&book.timestamp)
        .bind(&book.symbol)
        .bind(&book.price)
        .execute(conn)
        .await;
       
}





#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let url = "postgres://postgres:sjualyoacp@localhost:5432/postgres";
   let pool = sqlx::postgres::PgPool::connect(url).await?;

   
   
   let streamer = Streamer::new(vec!["INFY.NS", "RELIANCE.NS", "IRCTC.NS", "TCS.NS"]);

  streamer.stream().await
     .for_each(|quote| {

       let sample = DataSample {
           timestamp: quote.timestamp,
           symbol: quote.symbol,
           price: quote.price
       };   
       create(&sample, &pool);
       println!("At {}, {} is trading for ${}", sample.timestamp, sample.symbol, sample.price);
        future::ready(())
     })
     .await;


   
   
   Ok(())
}
//cargo run -q


//https://www.youtube.com/watch?v=TCERYbgvbq0



//use postgres::{Client, NoTls, Error};






































