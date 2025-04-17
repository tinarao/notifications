use dotenv::dotenv;

mod notifications;
mod telegram;
use tbot::types::chat::Id;
use telegram::TelegramNotificator;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut telegram = TelegramNotificator::new();

    // builder example
    // let ntf = NotificationBuilder::new()
    //     .text("Test Notification".to_string())
    //     .kind(NotificationKind::Daily)
    //     .period("".to_string())
    //     .build();

    let test_id = 123456789;
    let i = Id::from(test_id);
    let _ = telegram.send("sosi", i).await;
}
