use axum::{Json, extract::State, http::StatusCode};

use crate::{
    AppState,
    notifications::{NotificationBuilder, NotificationKind, NotificationPlatform},
    utils::{ResponseFabric, rfc3339_to_local},
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
pub struct MessageResponse {
    pub message: String,
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
) -> (StatusCode, Json<MessageResponse>) {
    let kind = match payload.is_daily {
        true => NotificationKind::Daily,
        false => NotificationKind::Instant,
    };

    if payload.is_daily && payload.daily_send_timestamps.len() == 0 {
        return ResponseFabric::bad_request(
            "is_daily is set true, but daily_send_timestamps size is 0",
        );
    }

    let platform = match parse_platform_from_request(payload.platform) {
        Ok(platform) => platform,
        Err(e) => {
            return ResponseFabric::bad_request(e.as_str());
        }
    };

    let send_to = payload.send_to.parse::<i64>().unwrap();
    let mut notification = NotificationBuilder::new()
        .text(payload.text)
        .kind(kind)
        .send_to(send_to)
        .platform(platform)
        .build();

    // add daily timestamps to notification if it's kind set to daily
    if payload.is_daily {
        for payload_ts in payload.daily_send_timestamps {
            let timestamp_utc = match rfc3339_to_local(&payload_ts) {
                Ok(t_utc) => t_utc,
                Err(_) => {
                    return ResponseFabric::bad_request(&format!(
                        "Incorrect date string: \"{}\". Expected format is RFC3339: 2025-04-18T12:00:00Z",
                        &payload_ts
                    ));
                }
            };

            match notification.add_daily_timestamp(timestamp_utc) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error adding daily timestamp: {}", e);
                    return ResponseFabric::bad_request(&format!(
                        "Error adding daily timestamp: {}",
                        e
                    ));
                }
            }
        }
    }

    match notification.kind {
        NotificationKind::Instant => {
            if let Err(e) = notification.send_instant(state.telegram).await {
                return ResponseFabric::internal_server_error(&format!(
                    "Failed to send notification: {}",
                    e
                ));
            }

            return ResponseFabric::ok("Sent!");
        }

        NotificationKind::Daily => {
            if let Err(e) = state.storage.persist_notification(&notification) {
                eprintln!("failed to persist notification: {}", e);
                return ResponseFabric::internal_server_error(
                    "Failed to save notification metadata",
                );
            }

            if let Err(e) = state.scheduler.add_notification(&notification) {
                println!("Failed to add notification to scheduler: {}", e);
                return ResponseFabric::internal_server_error(
                    "Failed to add notification to scheduler",
                );
            }

            return ResponseFabric::ok("Notification metadata successfully saved");
        }
    }
}
