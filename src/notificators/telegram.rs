use crate::{notifications::Notification, notificators::Notificator};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use teloxide::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactData {
    pub user_id: i64,
}

pub struct TelegramNotificator {
    bot: Arc<Bot>,
}

impl TelegramNotificator {
    pub fn new(token: String) -> Self {
        let bot = Bot::new(token);
        Self { bot: Arc::new(bot) }
    }
}

impl Notificator for TelegramNotificator {
    async fn send(&self, notification: &Notification) -> Result<(), String> {
        let chat_id = notification.send_to.user_id;
        self.bot
            .send_message(ChatId(chat_id), &notification.text)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
