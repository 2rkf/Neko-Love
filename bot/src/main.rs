#![allow(deprecated)]
#![allow(warnings)]

mod commands;
mod events;
mod shard;

use std::{env, error::Error, sync::Arc};
use twilight_gateway::{Event, EventTypeFlags, Shard, ShardId, StreamExt};
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::ApplicationMarker, Id};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").map_err(|_| "Missing DISCORD_TOKEN in .env file")?;
    let application_id = env::var("DISCORD_APPLICATION_ID")
        .map_err(|_| "Missing DISCORD_APPLICATION_ID in .env file")?
        .parse::<u64>()
        .map_err(|_| "DISCORD_APPLICATION_ID must be a valid u64")?;

    let http = Arc::new(HttpClient::new(token.clone()));
    let application_id = Id::new(application_id);
    let mut shard = shard::create_shard(token)?;

    commands::register_commands(&http, application_id).await?;

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");
            continue;
        };

        tokio::spawn(events::handle_event(event, Arc::clone(&http), application_id));
    }

    Ok(())
}
