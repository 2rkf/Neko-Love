use std::sync::Arc;
use twilight_gateway::Event;
use twilight_http::Client as HttpClient;
use twilight_model::{
    application::interaction::{InteractionData, InteractionType},
    gateway::payload::incoming::InteractionCreate,
    id::{marker::ApplicationMarker, Id},
};
use crate::commands;

pub async fn handle_interaction_create(
    interaction: InteractionCreate,
    http: Arc<HttpClient>,
    application_id: Id<ApplicationMarker>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let interaction = interaction.0;

    if interaction.kind != InteractionType::ApplicationCommand {
        return Ok(());
    }

    if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        commands::handle_command(
            &data.name,
            &http,
            interaction.id,
            &interaction.token,
        ).await?;
    }

    Ok(())
}