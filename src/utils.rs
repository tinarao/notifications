use axum::{Json, http::StatusCode};
use chrono::{DateTime, Local};

use crate::endpoints::MessageResponse;

pub fn rfc3339_to_local(rfc3339: &str) -> Result<DateTime<Local>, String> {
    let dt = DateTime::parse_from_rfc3339(rfc3339)
        .map(|dt| dt.with_timezone(&Local))
        .map_err(|e| e.to_string());

    return dt;
}

pub struct ResponseFabric {}

impl ResponseFabric {
    pub fn bad_request(message: &str) -> (StatusCode, Json<MessageResponse>) {
        return (
            StatusCode::BAD_REQUEST,
            Json(MessageResponse {
                message: message.to_string(),
            }),
        );
    }

    pub fn internal_server_error(message: &str) -> (StatusCode, Json<MessageResponse>) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: message.to_string(),
            }),
        );
    }

    pub fn ok(message: &str) -> (StatusCode, Json<MessageResponse>) {
        return (
            StatusCode::OK,
            Json(MessageResponse {
                message: message.to_string(),
            }),
        );
    }
}
