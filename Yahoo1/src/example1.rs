use postgres::{Client, NoTls, Error};

use futures::{ future, StreamExt };
use yahoo_finance::Streamer;

struct LiveData {
    timestamp: i32,
    symbol: String,
    price: String
}

#[tokio::main]
async fn main()  -> Result<(), Error> {
    let mut client = Client::connect("host=localhost user=postgres password=sjualyoacp", NoTls)?;

let streamer = Streamer::new(vec!["INFY.NS", "RELIANCE.NS", "IRCTC.NS", "TCS.NS"]);

   streamer.stream().await
      .for_each(|quote| {

       
            let author = LiveData {
                timestamp: quote.timestamp,
                symbol: quote.symbol,
                price: quote.price
            };


            client.execute(
               
                "INSERT INTO LiveData1 (timestamp, symbol, price) VALUES ($1, $2, $3)",
                &[&author.timestamp, &author.symbol, &author.price],
        )?;
         println!("At {}, {} is trading for ₹{}", quote.timestamp, quote.symbol, quote.price);
         future::ready(())
      })
      .await;

    

       
    

    for row in client.query("SELECT * FROM LiveData1", &[])? {
        let author = LiveData {
            timestamp: row.get(0),
            symbol: row.get(1),
            price: row.get(2),
        };
        println!("Author {} is from {}", author.timestamp, author.symbol, author.price);
    }

    Ok(())

}