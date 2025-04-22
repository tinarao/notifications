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
- `MODE`

### Mode
`MODE` environment variable sets an mode, in which app runs. It can be "native" or "docker" and for now affects only connection string for redis. If not set, defaults to "docker".

## Usage

Keep in mind, that service is in active development.

### Prerequisites

- Docker and Docker Compose (for containerized deployment)
- Telegram Bot Token

### Natively

1. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your Telegram bot token
   ```

2. Run the service:
   ```bash
   cargo run
   ```

### Docker-compose

Since notificator requires Redis Stack for fast JSON storage, you need to add one to your docker-compose file.

First, clone the repo and build the image
```bash
    git clone https://github.com/tinarao/notificator.git notificator && cd notificator
    docker build -t notificator .
    
```

Then, add following to your docker-compose.yml

```yaml
redis:
    image: redis/redis-stack:latest
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-stack-server
    healthcheck:
      test: [ "CMD", "redis-cli", "ping" ]
      interval: 5s
      timeout: 3s
      retries: 3

notificator:
    image: notificator:latest
    build:
      context: .
      dockerfile: Dockerfile
    depends_on:
      redis:
        condition: service_healthy
    ports:
      - "3692:3692"
    environment:
      - TELEGRAM_BOT_TOKEN=${TELEGRAM_BOT_TOKEN}
    healthcheck:
      test: [ "CMD", "curl", "-f", "http://localhost:3692/hc" ]
      interval: 30s
      timeout: 3s
      retries: 3
```

## API Documentation

### Healthcheck

**Endpoint:** `GET /hc`

Should return "Alive" string with status 200.

### Find by key
**Endpoint:** `GET /find/:notification_key`

**Request Body:**
```json
{
  "message": "Found",
  "notification": {
    "uuid": "random uuid key",
    "text": "Default notification",
    "daily_send_timestamps": [],
    "kind": "Instant",
    "platform": "Telegram",
    "send_to": {
      "user_id": 0
    },
    "last_sent": null,
    "created_at": "2025-04-22 14:14:40.007832589 +03:00"
  }
}
```

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
- Maybe web interface ?? for notification management
