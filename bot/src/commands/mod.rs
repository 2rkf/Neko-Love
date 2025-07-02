mod about;
mod ping;

use twilight_http::Client as HttpClient;
use twilight_model::{
    application::command::Command,
    id::{marker::ApplicationMarker, Id},
};

pub async fn register_commands(
    http: &HttpClient,
    application_id: Id<ApplicationMarker>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let commands = vec![
        ping::register(application_id),
        about::register(application_id),
    ];

    http.interaction(application_id)
        .set_global_commands(&commands)
        .await?;

    tracing::info!("Registered slash commands.");
    Ok(())
}

pub async fn handle_command(
    command_name: &str,
    http: &HttpClient,
    interaction_id: Id<twilight_model::id::marker::InteractionMarker>,
    interaction_token: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match command_name {
        "ping" => ping::handle(http, interaction_id, interaction_token).await,
        "about" => about::handle(http, interaction_id, interaction_token).await,
        _ => {
            tracing::warn!("Unknown command: {}", command_name);
            Err("Unknown command".into())
        }
        _ => Ok(()),
    }
}
