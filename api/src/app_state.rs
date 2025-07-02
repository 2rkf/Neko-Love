use anyhow::Result;
use bytes::Bytes;
use moka::future::Cache;
use std::path::PathBuf;
use std::sync::Arc;
use sqlx::MySqlPool;

use crate::services::image_service::ImageService;

/// Shared application state containing database pool, cache, and services
#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,            // Database connection pool
    pub cache: Arc<Cache<String, Bytes>>,  // Existing cache
    pub image_service: Arc<ImageService>,  // Existing image service
}

/// Creates a new shared AppState with database pool, assets path, and base URL
pub fn create_state(
    pool: MySqlPool,
    assets_path: PathBuf,
    base_url: String,
) -> Result<AppState> {
    let image_service = Arc::new(ImageService::new(assets_path.clone(), base_url)?);

    let cache = Arc::new(
        Cache::builder()
            .max_capacity(1000)
            .time_to_live(std::time::Duration::from_secs(300))
            .build(),
    );

    Ok(AppState {
        pool,
        cache,
        image_service,
    })
}