use crate::storage::Storage;
use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use notificators::TelegramNotificator;
use scheduler::Scheduler;
use std::{env, sync::Arc};

mod endpoints;
mod notifications;
mod notificators;
mod scheduler;
mod storage;
mod utils;

const DEFAULT_PORT: i16 = 3692;

#[derive(Clone)]
pub struct AppState {
    telegram: Arc<TelegramNotificator>,
    storage: Arc<Storage>,
    scheduler: Arc<Scheduler>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();
    let storage = Storage::new();

    let tg_token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(t) => t,
        Err(e) => {
            panic!("{}", e)
        }
    };

    let telegram_notificator = Arc::new(TelegramNotificator::new(tg_token));
    let scheduler = Scheduler::new(telegram_notificator.clone());

    let state = AppState {
        telegram: telegram_notificator,
        storage: Arc::new(storage),
        scheduler: Arc::new(scheduler),
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
