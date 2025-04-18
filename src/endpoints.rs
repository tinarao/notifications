use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::Utc;

use crate::{
    AppState,
    notifications::{NotificationBuilder, NotificationKind, NotificationPlatform},
};

const ALLOWED_PLATFORMS: [&str; 1] = ["telegram"];

#[derive(serde::Deserialize)]
pub struct RegisterNotificationMetadata {
    pub text: String,
    pub daily_send_timestamps: Vec<String>,
    pub is_daily: bool,
    pub platform: String,
    pub send_to: String,
}

#[derive(serde::Serialize)]
struct MessageResponse {
    message: String,
}

fn parse_platform_from_request(input: String) -> Result<NotificationPlatform, String> {
    let normalized = input.to_lowercase();
    let is_valid = ALLOWED_PLATFORMS.contains(&input.as_str());
    if !is_valid {
        return Err("Incorrect platform. Supported are \"telegram\" & \"email\"".to_string());
    }

    return match normalized.as_str() {
        "telegram" => Ok(NotificationPlatform::Telegram),
        "email" => Ok(NotificationPlatform::Email),
        _ => Err("Unsupported platform.".to_string()),
    };
}

#[axum::debug_handler]
pub async fn register_notification_metadata(
    State(state): State<AppState>,
    Json(payload): Json<RegisterNotificationMetadata>,
) -> impl IntoResponse {
    let kind = match payload.is_daily {
        true => NotificationKind::Daily,
        false => NotificationKind::Instant,
    };

    if payload.is_daily && payload.daily_send_timestamps.len() == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(MessageResponse {
                message: "is_daily is set true, but daily_send_timestamps size is 0".to_string(),
            }),
        );
    }

    let platform = match parse_platform_from_request(payload.platform) {
        Ok(platform) => platform,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(MessageResponse { message: e }),
            );
        }
    };

    let mut notification = NotificationBuilder::new()
        .text(payload.text)
        .kind(kind)
        .platform(platform)
        .build();

    let send_to = payload.send_to.parse::<i64>().unwrap();
    notification.add_send_to(send_to);

    if payload.is_daily {
        for payload_ts in payload.daily_send_timestamps {
            let timestamp_utc = match chrono::DateTime::parse_from_rfc3339(&payload_ts)
                .map(|dt| dt.with_timezone(&Utc))
            {
                Ok(t_utc) => t_utc,
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(MessageResponse {
                            message: format!(
                                "Incorrect date string: \"{}\". Expected format is RFC3339: 2025-04-18T12:00:00Z",
                                &payload_ts
                            ),
                        }),
                    );
                }
            };

            match notification.add_daily_timestamp(timestamp_utc) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error adding daily timestamp: {}", e);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(MessageResponse { message: e }),
                    );
                }
            }
        }
    }

    match notification.kind {
        NotificationKind::Instant => {
            if let Err(e) = notification.send(state.telegram).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MessageResponse {
                        message: format!("Error sending notification: {}", e),
                    }),
                );
            }

            return (
                StatusCode::OK,
                Json(MessageResponse {
                    message: "Sent!".to_string(),
                }),
            );
        }

        NotificationKind::Daily => {
            if let Err(e) = state.storage.persist_notification(&notification) {
                println!("Failed to save notification metadata: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(MessageResponse {
                        message: "Failed to save notification metadata".to_string(),
                    }),
                );
            }

            return (
                StatusCode::OK,
                Json(MessageResponse {
                    message: "Notification metadata successfully saved".to_string(),
                }),
            );
        }
    }
}
