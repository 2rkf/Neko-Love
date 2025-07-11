use anyhow::{Context, Result};
use aws_sdk_s3::Client;
use rand::seq::IndexedRandom;
use std::sync::Arc;

/// Service for managing and serving images from the assets directory.
pub struct ImageService {
    base_url: String,
    s3_client: Arc<Client>,
    bucket: String,
}

impl ImageService {
    /// Creates a new ImageService with the S3 client and base URL.
    pub fn new(base_url: String, s3_client: Arc<Client>, bucket: String) -> Self {
        Self {
            base_url,
            s3_client,
            bucket,
        }
    }

    /// Builds a full URL for an image given its category and filename.
    pub fn build_image_url(&self, filename: &str) -> String {
        format!("{}/img/{}", self.base_url, filename)
    }

    /// Gets a random image from the specified category.
    pub async fn get_random_image(
        &self,
        content_type: &str,
        category: &str,
    ) -> Result<(String, String)> {
        let content_dir = match content_type {
            "sfw" | "nsfw" => content_type,
            _ => anyhow::bail!("Invalid content type. Must be 'sfw' or 'nsfw'"),
        };

        if category.contains("..") || category.contains('/') || category.contains('\\') {
            anyhow::bail!("Invalid category name: {}", category);
        }

        let prefix = format!("assets/{}/{}/", content_dir, category);
        let response = self
            .s3_client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(&prefix)
            .send()
            .await
            .context("Failed to list objects from S3")?;

        let mut images = Vec::new();
        if let Some(contents) = response.contents {
            for obj in contents {
                if let Some(key) = obj.key {
                    if key.ends_with(".jpg")
                        || key.ends_with(".png")
                        || key.ends_with(".jpeg")
                        || key.ends_with(".gif") | key.ends_with(".webp")
                    {
                        let filename = key
                            .rsplit('/')
                            .next()
                            .ok_or_else(|| anyhow::anyhow!("Invalid key format"))?
                            .to_string();
                        let id = filename
                            .rsplit_once('.')
                            .map(|(stem, _)| stem.to_string())
                            .unwrap_or(filename.clone());
                        images.push((id, filename));
                    }
                }
            }
        }

        if images.is_empty() {
            anyhow::bail!("No images found in category: {}", category);
        }

        let mut rng = rand::rng();
        images
            .choose(&mut rng)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Failed to select random image"))
    }
}
