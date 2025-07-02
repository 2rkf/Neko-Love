use twilight_model::gateway::payload::incoming::Ready;

pub async fn handle_ready(ready: Ready) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("{} is awake.", ready.user.name);
    Ok(())
}
