use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum NotificationKind {
    Daily,
    AtDemand,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub uuid: String,
    pub text: String,
    pub period: Option<String>,
    pub kind: NotificationKind,
    pub created_at: String,
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
            period: None,
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

    pub fn period(mut self, period: String) -> NotificationBuilder {
        self.notification.period = Some(period);
        return self;
    }

    pub fn build(self) -> Notification {
        return self.notification;
    }
}
