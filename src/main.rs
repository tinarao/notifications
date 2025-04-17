use dotenv::dotenv;
use notifications::NotificationBuilder;
use storage::Storage;

mod notifications;
mod storage;
mod telegram;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let ntf = NotificationBuilder::new()
        .kind(notifications::NotificationKind::Daily)
        .build();

    let uuid = ntf.uuid.clone();

    let storage = Storage::new();

    match storage.set_notification(&uuid, &ntf) {
        Ok(_) => {}
        Err(e) => println!("Error storing notification: {}", e),
    }

    match storage.get_notification(&uuid) {
        Ok(notification) => println!("Retrieved notification: {:#?}", notification),
        Err(e) => println!("Error retrieving notification: {}", e),
    }
}
