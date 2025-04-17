use redis::{Commands, Connection, JsonCommands, RedisError};
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

    pub fn ping(&self) -> Result<(), String> {
        let mut con = self.get_conn().map_err(|e| e.to_string())?;
        return con.ping().map_err(|e| e.to_string());
    }

    pub fn get_conn(&self) -> Result<Connection, RedisError> {
        return self.client.get_connection();
    }

    pub fn persist_notification(
        &self,
        key: &str,
        notification: &Notification,
    ) -> Result<(), String> {
        let mut con = self.get_conn().map_err(|e| e.to_string())?;

        con.json_set::<_, _, _, ()>(key, JSON_NOTIFICATION_KEY, notification)
            .map_err(|e| format!("Failed to set JSON value: {}", e))?;

        return Ok(());
    }

    pub fn get_notification(&self, key: &str) -> Result<Notification, String> {
        let mut con = self.get_conn().map_err(|e| e.to_string())?;

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

    pub fn delete_notification(&self, key: &str) -> Result<(), String> {
        let mut con = self.get_conn().map_err(|e| e.to_string())?;

        con.del::<_, ()>(key)
            .map_err(|e| format!("Failed to delete key: {}", e))?;

        return Ok(());
    }

    pub fn exists(&self, key: &str) -> Result<bool, String> {
        let mut con = self.get_conn().map_err(|e| e.to_string())?;

        return con
            .exists(key)
            .map_err(|e| format!("Failed to check key existence: {}", e));
    }
}
