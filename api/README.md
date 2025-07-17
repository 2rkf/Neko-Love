# Nekoi API

A high-performance API for serving and managing anime-themed images (SFW & NSFW), user authentication, and web-related functionalities. Built with the **Axum** web framework and backed by **MariaDB** for data persistence.

## Features
- Serve categorised anime images (SFW/NSFW) from an S3-compatible object storage cloud.
- User authentication and session management.
- Lightweight and fast due to Rust's Axum framework.
- Configurable database, asset storage, and Redis integration.

---

## Setup Guide

### Prerequisites
- **Rust** (stable version, `1.88.0` recommended)
- **MariaDB** (or MySQL-compatible database)
- A folder for storing image assets (see below)

---

### Asset Storage Setup
All images are stored in an S3-compatible object storage cloud (e.g., Cloudflare R2) under a designated bucket. Follow these steps to set up:

1. **Create the `assets` folder** inside the S3 bucket.
2. **Add subdirectories** for content types:
   - `sfw/` (Safe For Work)
   - `nsfw/` (Not Safe For Work)
3. **Organise images into categories** by creating subfolders (e.g., `neko`, `azurlane`).

#### Example Structure:
```plaintext
nekoi/                            # Main bucket
├── assets/
│   ├── nsfw/                   # NSFW content
│   │   └── azurlane/               # Example category
│   │       └── 20250622552061.jpg    # Image files
│   └── sfw/                    # SFW content
│       └── neko/
│           └── 20250621449271.png
```

4. **Update the `CATEGORIES` field** in the `.env` file to ensure the values correspond with the categories defined in the S3 bucket.

> **Note**: The API will only serve images from preconfigured categories. Ensure filenames are URL-safe (no spaces or special characters). Configure appropriate permissions in your S3-compatible storage to allow API access to the bucket.

### Database Configuration

The API uses **MariaDB** for user data sessions.

1. Set up a database:
    - Install MariaDB (e.g., `sudo apt install mariadb-server` on Debian/Ubuntu).
    - Create a database and user:
    ```sql
    CREATE DATABASE nekoi;
    CREATE USER 'neko_user'@'localhost' IDENTIFIED BY 'secure_password';
    GRANT ALL PRIVILEGES ON nekoi.* TO 'neko_user'@'localhost';
    FLUSH PRIVILEGES;
    ```
2. Configure `.env`:

    Add your database URL to the `.env` file:
    ```env
    DATABASE_URL=mysql://neko_user:secure_password@localhost/nekoi
    ```
3. Run migrations:

    Use `sqlx-cli` to apply schema migrations:
    ```bash
    sqlx migrate run
    ```

### Redis Configuration

The API uses **Redis** to store users' rate limits.

1. Install Redis:
    - On Debian/Ubuntu:
    ```bash
    sudo apt install redis-server
    ```
    - On macOS (using Homebrew):
    ```bash
    brew install redis
    ```
    - Ensure Redis is running:
    ```bash
    redis-server
    ```
2. Configure `.env`:
    - Add your Redis connection details to the `.env` file:
    ```env
    REDIS_URL=redis://:secure_password@localhost:6379
    ```
3. Verify Redis Connection:
    - Test the connection using the Redis CLI:
    ```bash
    redis-cli ping
    ```
    If successful, it should return `PONG`.

### Running the API

1. Build and run in development mode:

    ```bash
    cargo run
    ```

2. For production, use a release build:

    ```bash
    cargo build --release && ./target/release/api
    ```

### API Endpoints

| Endpoint | Method | Description |
| --- | --- | --- |
| `/api/v1/sfw/neko` | **GET** | Fetch a random SFW neko image. |
| `/api/v1/nsfw/azurlane` | **GET** | Fetch a random NSFW Azur Lane image. |
