use crate::storage::Storage;
use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use std::{env, sync::Arc};
use telegram::TelegramNotificator;

mod endpoints;
mod notifications;
mod storage;
mod telegram;

const DEFAULT_PORT: i16 = 3692;

#[derive(Clone)]
pub struct AppState {
    telegram: Arc<TelegramNotificator>,
    storage: Arc<Storage>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();
    let storage = Storage::new();

    match storage.ping() {
        Ok(_) => println!("Connected to Redis"),
        Err(e) => println!("Error connecting to Redis: {}", e),
    };

    let state = AppState {
        telegram: Arc::new(TelegramNotificator::new()),
        storage: Arc::new(storage),
    };

    let router = Router::new()
        .route("/hc", get(|| async { "Alive!" }))
        .route(
            "/notifications",
            post(endpoints::register_notification_metadata),
        )
        .with_state(state);

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
