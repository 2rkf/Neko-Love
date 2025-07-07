# Neko-Love API

A high-performance API for serving and managing anime-themed images (SFW & NSFW), user authentication, and web-related functionalities. Built with the **Axum** web framework and backed by **MariaDB** for data persistence.

## Features
- Serve categorized anime images (SFW/NSFW) from a local filesystem.
- User authentication and session management.
- Lightweight and fast due to Rust's Axum framework.
- Configurable database and asset storage.

---

## Setup Guide

### Prerequisites
- **Rust** (stable version, `1.88.0` recommended)
- **MariaDB** (or MySQL-compatible database)
- A folder for storing image assets (see below)

---

### Asset Directory Setup
All images are stored in a physical directory (`/assets`), which must be manually created. Follow these steps:

1. **Create the `assets` folder** inside the `/api` directory.
2. **Add subdirectories** for content types:
   - `sfw/` (Safe For Work)
   - `nsfw/` (Not Safe For Work)
3. **Organise images into categories** by creating subfolders (e.g., `neko`, `azurlane`).

#### Example Structure:
```plaintext
api/
├── assets/
│   ├── nsfw/                  # NSFW content
│   │   └── bdsm/              # Example category
│   │       └── image-01.jpg   # Image files
│   └── sfw/                   # SFW content
│       └── neko/
│           └── neko-01.png
```

> **Note**: The API will only serve images from preconfigured categories. Ensure filenames are URL-safe (no spaces/special chars).

### Database Configuration

The API uses **MariaDB** for user data sessions.

1. Set up a database:
    - Install MariaDB (e.g., `sudo apt install mariadb-server` on Debian/Ubuntu).
    - Create a database and user:
    ```sql
    CREATE DATABASE neko-love;
    CREATE USER 'neko_user'@'localhost' IDENTIFIED BY 'secure_password';
    GRANT ALL PRIVILEGES ON neko-love.* TO 'neko_user'@'localhost';
    FLUSH PRIVILEGES;
    ```
2. Configure `.env`:

    Add your database URL to the `.env` file:
    ```env
    DATABASE_URL=mysql://neko_user:secure_password@localhost/neko-love
    ```
3. Run migrations:

    Use `sqlx-cli` to apply schema migrations:
    ```bash
    sqlx migrate run
    ```

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
