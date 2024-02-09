use futures::{ future, StreamExt };
use yahoo_finance::Streamer;

#[tokio::main]
async fn main() {
    let streamer = Streamer::new(vec!["INFY.NS", "RELIANCE.NS", "IRCTC.NS", "TCS.NS"]);

   streamer.stream().await
      .for_each(|quote| {
         println!("At {}, {} is trading for ${}", quote.timestamp, quote.symbol, quote.price);
         future::ready(())
      })
      .await;
}




//yahoo-finance = "0.3.0"
//futures = "0.3.30"