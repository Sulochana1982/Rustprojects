
use actix_web::{HttpServer, App, Web, Responder};
use std::io;
use std::chrono::Duration;

async fn status()-> impl Responder {
    "{\"status\": \"UP\"}"
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
   println!("Starting server at http://127.0.0.1:8080/");
   HttpServer::new(|| {
           App::new()
           .route("/", web::get().to(status))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
