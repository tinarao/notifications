use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::telegram::{self, TelegramNotificator};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum NotificationKind {
    Daily,
    Instant,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum NotificationPlatform {
    Telegram,
    Email,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    // Base ID
    pub uuid: String,

    // Preformatted message text
    pub text: String,

    // Array of stringified UTC dates
    // Max size = 2
    // Used if kind == NotificationKind::Daily
    pub daily_send_timestamps: Vec<String>,

    // Daily notifications sends every day
    // on timestamps, specified in self.daily_send_timestamps
    pub kind: NotificationKind,

    // Pretty much speaks for itself
    pub platform: NotificationPlatform,

    // Data, needed to send a message to certain person
    // for now there ContactData only for Telegram
    // Gonna implement one for Email too
    pub send_to: telegram::ContactData,

    pub last_sent: Option<String>, // Stringified UTC date
    pub created_at: String,        // Stringified UTC date
}

pub const JSON_NOTIFICATION_KEY: &str = "$";
const MAX_DAILY_TIMESTAMPS: usize = 2;

impl Notification {
    pub fn default() -> Self {
        let uuid = Uuid::new_v4();

        return Notification {
            uuid: uuid.to_string(),
            kind: NotificationKind::Instant,
            platform: NotificationPlatform::Telegram,
            send_to: telegram::ContactData { user_id: 0 },
            created_at: chrono::Utc::now().to_string(),
            text: "Default notification".to_string(),
            daily_send_timestamps: Vec::new(),
            last_sent: None,
        };
    }

    pub fn add_send_to(&mut self, send_to: i64) {
        self.send_to = telegram::ContactData { user_id: send_to };
    }

    pub fn add_daily_timestamp(&mut self, timestamp: DateTime<Utc>) -> Result<(), String> {
        if self.daily_send_timestamps.len() >= MAX_DAILY_TIMESTAMPS {
            return Err("You can only send 2 notification a day".to_string());
        }

        self.daily_send_timestamps.push(timestamp.to_string());
        Ok(())
    }

    // Колхозный dependency injection
    // TODO: элегантнее
    pub async fn send(&self, bot: Arc<TelegramNotificator>) -> Result<(), String> {
        match self.kind == NotificationKind::Instant {
            true => match self.send_instant(bot).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            false => Err("Daily notifications are not implemented yet.".to_string()),
        }
    }

    async fn send_instant(&self, bot: Arc<TelegramNotificator>) -> Result<(), String> {
        let bot = bot.clone();
        match self.platform {
            NotificationPlatform::Telegram => match bot.send(self).await {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to send notification".to_string()),
            },
            NotificationPlatform::Email => {
                return Err("Email notifications are not implemented yet.".to_string());
            }
        }
    }
}

// Builder

pub struct NotificationBuilder {
    notification: Notification,
}

impl NotificationBuilder {
    pub fn new() -> Self {
        return NotificationBuilder {
            notification: Notification::default(),
        };
    }

    pub fn kind(mut self, kind: NotificationKind) -> NotificationBuilder {
        self.notification.kind = kind;
        return self;
    }

    pub fn text(mut self, text: String) -> NotificationBuilder {
        self.notification.text = text;
        return self;
    }

    pub fn daily_send_timestamp(mut self, timestamp: chrono::DateTime<Utc>) -> NotificationBuilder {
        if self.notification.daily_send_timestamps.len() >= MAX_DAILY_TIMESTAMPS {
            return self;
        }

        self.notification
            .daily_send_timestamps
            .push(timestamp.to_string());

        return self;
    }

    pub fn send_to(mut self, send_to: telegram::ContactData) -> NotificationBuilder {
        self.notification.send_to = send_to;
        return self;
    }

    pub fn platform(mut self, platform: NotificationPlatform) -> NotificationBuilder {
        self.notification.platform = platform;
        return self;
    }

    pub fn build(self) -> Notification {
        return self.notification;
    }
}

// general methods
