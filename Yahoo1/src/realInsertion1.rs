//https://www.youtube.com/watch?v=TCERYbgvbq0



use postgres::{Client, NoTls, Error};

use futures::{ future, StreamExt };
use yahoo_finance::Streamer;
use std::error::Error;


struct DataSample{
    pub timestamp: String,
    pub symbol: String,
    pub price: f64,
 }
 
 async fn create(sample: &DataSample, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>>
 {
    
    let query = " INSERT INTO dataSample (timestamp, symbol, price) VALUES ($1, $2, $3)";
    sqlx::query(query)
         .bind(&sample.timestamp)
         .bind(&sample.symbol)
         .bind(&sample.price)
         .execute(conn)
         .await?;
 
        
         Ok(())
 }
 


 #[tokio::main]
 async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:sjualyoacp@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
     
   sqlx::migrate!("./migrations").run(&pool).await?;

let streamer = Streamer::new(vec!["INFY.NS", "RELIANCE.NS", "IRCTC.NS", "TCS.NS"]);

streamer.stream().await
   .for_each(|quote| {

       let sample = DataSample {
          timestamp: &quote.timestamp1,
          symbol: &quote.symbol,
          price: quote.price
      };     

      //println!("At {}, {} is trading for ₹{}", sample.timestamp, sample.symbol, sample.price);
      create(&sample, &pool).await?;

      //println!("At {}, {} is trading for ₹{}", quote.timestamp, quote.symbol, quote.price);
      future::ready(())
   })
   .await;
   Ok(())
}
