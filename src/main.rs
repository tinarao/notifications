use crate::storage::Storage;
use axum::{Router, routing::get};
use dotenv::dotenv;
use std::env;

mod notifications;
mod storage;
mod telegram;

const DEFAULT_PORT: i16 = 3692;

#[tokio::main]
async fn main() {
    // Setup
    dotenv().ok();
    let storage = Storage::new();

    match storage.ping() {
        Ok(_) => println!("Connected to Redis"),
        Err(e) => println!("Error connecting to Redis: {}", e),
    }

    let router = Router::new().route("/hc", get(|| async { "Alive!" }));

    let port = match env::var("PORT") {
        Ok(v) => v,
        Err(_) => {
            println!("Port is not specified. Run on default :{}", DEFAULT_PORT);
            DEFAULT_PORT.to_string()
        }
    };
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
