use dotenv::dotenv;

mod telegram;
use tbot::types::chat::Id;
use telegram::TelegramNotificator;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut telegram = TelegramNotificator::new();

    let i = Id::from(217312859);
    telegram.send("sosi", i).await;
}
