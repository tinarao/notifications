use chrono::{DateTime, Local, Timelike};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::notifications::Notification;
use crate::notificators::TelegramNotificator;

pub struct Scheduler {
    tx: mpsc::Sender<Notification>,
}

/*
    Получилось сложнее, чем должно было быть.
    1. Создание задачи для каждого времени отправки
    1.1 Вычисление времени отправки
    1.2 Если время прошло, отправляем на следующий день
    1.3 Если время не наступило, планируем на сегодня
    1.4 Отправляем уведомляху
*/

impl Scheduler {
    pub fn new(telegram: Arc<TelegramNotificator>) -> Self {
        let (tx, mut rx): (mpsc::Sender<Notification>, mpsc::Receiver<Notification>) =
            mpsc::channel(32);
        let telegram_clone = telegram.clone();

        tokio::spawn(async move {
            // Ожидание новых уведомлений и обработка в лупе
            while let Some(notification) = rx.recv().await {
                for timestamp_str in &notification.daily_send_timestamps {
                    if let Ok(dt) = DateTime::<Local>::from_str(timestamp_str) {
                        let time = dt.time();
                        let hour = time.hour();
                        let minute = time.minute();
                        let notification_clone = notification.clone();
                        let telegram_clone = telegram_clone.clone();

                        tokio::spawn(async move {
                            loop {
                                let now = Local::now();
                                let target = now.date_naive().and_time(
                                    chrono::NaiveTime::from_hms_opt(hour, minute, 0).unwrap(),
                                );

                                let duration = if now.naive_local() > target {
                                    // Ждём до завтра
                                    let tomorrow = now.date_naive().succ_opt().unwrap();
                                    let next_target = tomorrow.and_time(
                                        chrono::NaiveTime::from_hms_opt(hour, minute, 0).unwrap(),
                                    );
                                    (next_target - now.naive_local()).to_std().unwrap()
                                } else {
                                    // Или ждём нужного времени сегодня
                                    (target - now.naive_local()).to_std().unwrap()
                                };

                                sleep(duration).await;

                                if let Err(e) = notification_clone
                                    .send_instant(telegram_clone.clone())
                                    .await
                                {
                                    eprintln!("Failed to send notification: {}", e);
                                }
                            }
                        });
                    }
                }
            }
        });

        Scheduler { tx }
    }

    pub fn add_notification(&self, notification: &Notification) -> Result<(), String> {
        println!("Adding notification to scheduler: {:?}", notification);
        self.tx
            .try_send((*notification).clone())
            .map_err(|e| e.to_string())
    }
}
