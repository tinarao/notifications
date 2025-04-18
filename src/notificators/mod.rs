use crate::notifications::Notification;

pub mod telegram;
pub use telegram::TelegramNotificator;

pub trait Notificator {
    async fn send(&self, notification: &Notification) -> Result<(), String>;
}
