use dotenv::dotenv;

mod notifications;
mod storage;
mod telegram;

#[tokio::main]
async fn main() {
    dotenv().ok();
}
