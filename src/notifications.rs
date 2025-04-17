use uuid::Uuid;

pub enum NotificationKind {
    Daily,
    AtDemand,
}

pub struct Notification {
    uuid: String,
    text: String,
    period: Option<String>,
    kind: NotificationKind,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Notification {
    pub fn new(kind: NotificationKind, text: String) -> Self {
        let uuid = Uuid::new_v4();
        return Notification {
            uuid: uuid.to_string(),
            kind: NotificationKind::AtDemand,
            created_at: chrono::Utc::now(),
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
