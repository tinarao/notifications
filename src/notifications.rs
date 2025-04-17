use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationKind {
    Daily,
    AtDemand,
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

    pub last_sent: Option<String>, // Stringified UTC date
    pub created_at: String,        // Stringified UTC date
}

pub const JSON_NOTIFICATION_KEY: &str = "$";

impl Notification {
    pub fn new(kind: NotificationKind, text: String) -> Self {
        let uuid = Uuid::new_v4();

        return Notification {
            uuid: uuid.to_string(),
            kind,
            created_at: chrono::Utc::now().to_string(),
            text,
            daily_send_timestamps: Vec::new(),
            last_sent: None,
        };
    }
}

// Builder

pub struct NotificationBuilder {
    notification: Notification,
}

impl NotificationBuilder {
    pub fn new() -> Self {
        return NotificationBuilder {
            notification: Notification::new(NotificationKind::Daily, "Empty".to_string()),
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
        if self.notification.daily_send_timestamps.len() >= 2 {
            return self;
        }

        self.notification
            .daily_send_timestamps
            .push(timestamp.to_string());

        return self;
    }

    pub fn build(self) -> Notification {
        return self.notification;
    }
}
