use axum::{Json, http::StatusCode};
use chrono::{DateTime, Local};

use crate::{
    endpoints::{MessageResponse, NotificationResponse},
    notifications::Notification,
};

pub trait Response {
    fn with_message(message: String) -> Self;
    fn with_existing(message: String, existing: Self) -> Self;
}

impl Response for MessageResponse {
    fn with_message(message: String) -> Self {
        Self {
            message,
            notification_id: "".to_string(),
        }
    }

    fn with_existing(message: String, existing: Self) -> Self {
        Self {
            message,
            notification_id: existing.notification_id,
        }
    }
}

impl Response for NotificationResponse {
    fn with_message(message: String) -> Self {
        Self {
            message,
            notification: Notification::default(),
        }
    }

    fn with_existing(message: String, existing: Self) -> Self {
        Self {
            message,
            notification: existing.notification,
        }
    }
}

pub fn rfc3339_to_local(rfc3339: &str) -> Result<DateTime<Local>, String> {
    let dt = DateTime::parse_from_rfc3339(rfc3339)
        .map(|dt| dt.with_timezone(&Local))
        .map_err(|e| e.to_string());

    return dt;
}

pub struct ResponseFabric {}

impl ResponseFabric {
    pub fn bad_request<T: Response>(message: &str) -> (StatusCode, Json<T>) {
        return (
            StatusCode::BAD_REQUEST,
            Json(T::with_message(message.to_string())),
        );
    }

    pub fn not_found<T: Response>(message: &str) -> (StatusCode, Json<T>) {
        return (
            StatusCode::NOT_FOUND,
            Json(T::with_message(message.to_string())),
        );
    }

    pub fn internal_server_error<T: Response>(message: &str) -> (StatusCode, Json<T>) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(T::with_message(message.to_string())),
        );
    }

    pub fn ok_with_existing<T: Response>(message: &str, existing: T) -> (StatusCode, Json<T>) {
        return (
            StatusCode::OK,
            Json(T::with_existing(message.to_string(), existing)),
        );
    }

    pub fn ok_with_id(
        message: &str,
        notification_id: String,
    ) -> (StatusCode, Json<MessageResponse>) {
        return (
            StatusCode::OK,
            Json(MessageResponse {
                message: message.to_string(),
                notification_id,
            }),
        );
    }
}
