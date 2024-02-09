// use futures::{ future, StreamExt };
// use yahoo_finance::Streamer;

// #[tokio::main]
// async fn main() {
//    let streamer = Streamer::new(vec!["INFY.NS", "RELIANCE.NS", "IRCTC.NS", "TCS.NS"]);

//    streamer.stream().await
//       .for_each(|quote| {
//          println!("At {}, {} is trading for ₹{}", quote.timestamp, quote.symbol, quote.price);
//          future::ready(())
//       })
//       .await;
// }




// use postgres::{Client, NoTls, Error};

// fn main() -> Result<(), Error> {
   
//     let mut client = Client::connect("host=localhost user=postgres password=sjualyoacp", NoTls)?;
    
//     client.batch_execute("
//         CREATE TABLE IF NOT EXISTS LiveData1 (
//             timestamp    VARCHAR NOT NULL,
//              symbol         VARCHAR NOT NULL,
//               price        VARCHAR NOT NULL
//             )
//     ")?;
//    println!("Table created");
//     Ok(())


// }



use postgres::{Client, NoTls, Error};

use futures::{ future, StreamExt };
use yahoo_finance::Streamer;

struct LiveData<'a> {
   timestamp: i64,
   symbol: & 'a str,
   price: f64
}
#[tokio::main]
async fn main()  
{
   let mut client = Client::connect("host=localhost user=postgres password=sjualyoacp", NoTls)?;

   let streamer = Streamer::new(vec!["INFY.NS", "RELIANCE.NS", "IRCTC.NS", "TCS.NS"]);

   streamer.stream().await
      .for_each(|quote| {

          let sample = LiveData {
             timestamp: quote.timestamp,
             symbol: &quote.symbol,
             price: quote.price
         };
          


          client.execute(
               
          "INSERT INTO LiveData1 (timestamp, symbol, price) VALUES ($1, $2, $3)",
          &[&sample.timestamp, &sample.symbol, &sample.price])?;

          println!("Inserted row");

         //println!("At {}, {} is trading for ₹{}", sample.timestamp, sample.symbol, sample.price);
      

         //println!("At {}, {} is trading for ₹{}", quote.timestamp, quote.symbol, quote.price);
         future::ready(())
      })
      .await;

   
 

   //       for row in client.query("SELECT * FROM LiveData1", &[])? {
       
   //      println!("Author {} is from {}", row.get(0), row.get(1), row.get(2));
   //  }

    

}