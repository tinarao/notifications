use redis::{Commands, Connection, JsonCommands};
use serde_json::Value;

use crate::notifications::{JSON_NOTIFICATION_KEY, Notification};

pub struct Storage {
    pub client: redis::Client,
}

impl Storage {
    pub fn new() -> Self {
        let client = match redis::Client::open("redis://127.0.0.1:6379/") {
            Ok(c) => c,
            Err(e) => panic!("failed to connect to redis: {}", e),
        };

        return Storage { client };
    }

    fn get_conn(&self) -> Result<Connection, String> {
        return match self.client.get_connection() {
            Ok(c) => Ok(c),
            Err(e) => {
                let msg = format!("Failed to get redis connection: {}", e);
                println!("{}", msg);
                return Err(msg);
            }
        };
    }

    pub fn persist_notification(&self, notification: &Notification) -> Result<(), String> {
        let mut con = self.get_conn()?;
        con.json_set::<_, _, _, ()>(&notification.uuid, JSON_NOTIFICATION_KEY, notification)
            .map_err(|e| format!("Failed to set JSON value: {}", e))?;

        return Ok(());
    }

    pub fn get_notification(&self, key: &str) -> Result<Notification, String> {
        let mut con = self.get_conn()?;

        let exists: bool = con
            .exists(key)
            .map_err(|e| format!("Failed to check key existence: {}", e))?;

        if !exists {
            return Err(format!("Key '{}' not found", key));
        }

        let result: String = con
            .json_get(key, JSON_NOTIFICATION_KEY)
            .map_err(|e| format!("Failed to get JSON value: {}", e))?;

        let value: Value =
            serde_json::from_str(&result).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let obj = if value.is_array() {
            value
                .as_array()
                .and_then(|arr| arr.first())
                .ok_or_else(|| "Invalid JSON structure".to_string())?
        } else {
            &value
        };

        let deserialized: Result<Notification, String> = serde_json::from_value(obj.clone())
            .map_err(|e| format!("Failed to deserialize JSON: {}", e));

        return deserialized;
    }

    pub fn get_all_notifications(&self) -> Result<Vec<Notification>, String> {
        let mut con = self.get_conn()?;

        // Get all keys
        let keys: Vec<String> = con
            .keys("*")
            .map_err(|e| format!("Failed to get keys: {}", e))?;

        let mut notifications = Vec::new();

        // Get notification for each key
        for key in keys {
            match self.get_notification(&key) {
                Ok(notification) => notifications.push(notification),
                Err(e) => eprintln!("Failed to get notification for key {}: {}", key, e),
            }
        }

        Ok(notifications)
    }

    pub fn delete_notification(&self, key: &str) -> Result<(), String> {
        let mut con = self.get_conn()?;

        con.del::<_, ()>(key)
            .map_err(|e| format!("Failed to delete key: {}", e))?;

        return Ok(());
    }

    pub fn exists(&self, key: &str) -> Result<bool, String> {
        let mut con = self.get_conn()?;

        return con
            .exists(key)
            .map_err(|e| format!("Failed to check key existence: {}", e));
    }
}
