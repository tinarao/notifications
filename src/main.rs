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

enum AppMode {
    Docker,
    Native,
}

fn get_app_mode() -> AppMode {
    match env::var("MODE") {
        Ok(s) => match s.trim().to_lowercase().as_str() {
            "docker" => AppMode::Docker,
            "native" => AppMode::Native,
            _ => {
                println!("invalid MODE env set. Setting mode to docker");
                AppMode::Docker
            }
        },
        Err(_) => {
            println!("MODE env not set. Setting mode to docker");
            AppMode::Docker
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let tg_token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            panic!("Telegram bot token environment variable is not set")
        }
    };

    let app_mode = get_app_mode();
    let storage = Storage::new(&app_mode);

    let telegram_notificator = Arc::new(TelegramNotificator::new(tg_token));
    let scheduler = Scheduler::new(telegram_notificator.clone());

    let state = AppState {
        telegram: telegram_notificator,
        storage: Arc::new(storage),
        scheduler: Arc::new(scheduler),
    };

    // schedule already registered notifications
    match state.storage.get_all_notifications() {
        Ok(notifications) => {
            println!("loaded {} notifications from storage", notifications.len());
            for notification in notifications {
                if let Err(e) = state.scheduler.add_notification(&notification) {
                    eprintln!(
                        "failed to register loaded notification with key {}: {}",
                        &notification.uuid, e
                    );
                };
            }
        }
        Err(e) => {
            eprintln!("Failed to load saved notifications: {}", e);
        }
    };

    let router = Router::new()
        .route("/hc", get(|| async { "Alive!" }))
        .route(
            "/notifications",
            post(endpoints::register_notification_metadata),
        )
        .route(
            "/find/:notification_key",
            get(endpoints::get_notification_metadata),
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
