use anyhow::{Context, Result};
use rand::seq::IndexedRandom;
use std::fs;
use std::path::{Path, PathBuf};

/// Service for managing and serving images from the assets directory
pub struct ImageService {
    assets_path: PathBuf,
    base_url: String,
}

impl ImageService {
    /// Creates a new ImageService with the given assets path and base URL
    pub fn new(assets_path: impl AsRef<Path>, base_url: String) -> Result<Self> {
        let assets_path = assets_path.as_ref().canonicalize().with_context(|| {
            format!("Failed to resolve assets path: {:?}", assets_path.as_ref())
        })?;

        if !assets_path.exists() {
            anyhow::bail!("Assets directory does not exist: {:?}", assets_path);
        }

        Ok(Self {
            base_url,
            assets_path,
        })
    }

    /// Builds a full URL for an image given its category and filename
    pub fn build_image_url(&self, filename: &str) -> String {
        format!("{}/img/{}", self.base_url, filename)
    }

    /// Gets a random image from the specified category
    pub fn get_random_image(&self, content_type: &str, category: &str) -> Result<(String, String)> {
         let content_dir = match content_type {
            "sfw" | "nsfw" => content_type,
            _ => anyhow::bail!("Invalid content type. Must be 'sfw' or 'nsfw'"),
        };

        if category.contains("..") || category.contains('/') || category.contains('\\') {
            anyhow::bail!("Invalid category name: {}", category);
        }

        let category_path = self.assets_path.join(content_dir).join(category);

        if !category_path.exists() {
            anyhow::bail!("Category directory does not exist: {:?}", category_path);
        }

        let entries = fs::read_dir(&category_path)?;

        let mut images = Vec::new();
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    let id = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default()
                        .to_string();
                    images.push((id, filename.to_string()));
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