# Notificator

<p align="center">
  <a>
    <img src="https://skillicons.dev/icons?i=rust,redis,docker" />
  </a>
</p>


A Rust-based notification service that handles both instant and scheduled daily notifications, currently supporting Telegram as the primary notification platform.

## Features

- Instant notifications
- Daily scheduled notifications (up to 2 times per day)
- Telegram integration
- REST API for notification management
- Persistent storage of notification settings

## How it works

The service consists of several key components:

### 1. Notification System

The core notification system supports two types of notifications:
- **Instant Notifications**: Sent immediately when requested
- **Daily Notifications**: Scheduled to be sent at specific times each day

Each notification contains:
- Unique identifier (UUID)
- Message text
- Platform (Telegram/Email)
- Target recipient information
- Timestamps for daily notifications
- Creation and last sent timestamps

### 2. Scheduler

The scheduler handles the timing and delivery of daily notifications:
- Creates separate tasks for each scheduled time
- Automatically adjusts for missed notifications
- Handles timezone-aware scheduling
- Supports multiple daily notifications (up to 2 per day)

### 3. Storage

The system maintains persistent storage for:
- Notification metadata
- Scheduling information
- User preferences
- Historical notification data

### 4. API Endpoints

The service exposes the following REST endpoints:
- `/hc` - Health check endpoint
- `/notifications` - Register new notification metadata

## Configuration

The service requires the following environment variables:
- `TELEGRAM_BOT_TOKEN` - Your Telegram bot token
- `PORT` - (Optional) Port to run the service on (default: 3692)

### Sending Notifications

- **Instant Notifications**: Sent immediately using `send_instant()`
- **Daily Notifications**: Automatically scheduled and sent at specified times

## Usage

Keep in mind, that service is in active development.

### Prerequisites

- Docker and Docker Compose (for containerized deployment)
- Telegram Bot Token

## Natively

1. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your Telegram bot token
   ```

2. Run the service:
   ```bash
   cargo run
   ```

Or using Docker:
   ```bash
   docker compose up
   ```

## Docker-compose

```yaml
services:
  notifications:
    image: your-registry/notifications:latest  # or use local build
    environment:
      - TELEGRAM_BOT_TOKEN=${TELEGRAM_BOT_TOKEN}
      - PORT=3692
    depends_on:
      - redis
    ports:
      - "3692:3692"

  redis:
    image: redis/redis-stack:latest
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-stack-server

volumes:
  redis_data:
```

## API Documentation

### Healthcheck

**Endpoint:** `POST /hc`

Should return "Alive" string with status 200.

### Register Notification

**Endpoint:** `POST /notifications`

**Request Body:**
```json
{
    "text": "Your notification message",
	"is_daily": false,
    "platform": "telegram",
    "send_to": "123456789", // stringified telegram chat id / email etc.
    "daily_send_timestamps": [
        "2024-04-19T09:00:00", // ISO Strings
        "2024-04-19T21:00:00"
    ]
}
```

## Development plan

- Email notification support
- Web interface for notification management