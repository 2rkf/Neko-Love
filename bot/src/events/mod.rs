pub mod interaction;
pub mod ready;

use std::sync::Arc;
use twilight_gateway::Event;
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::ApplicationMarker, Id};

pub async fn handle_event(
    event: Event,
    http: Arc<HttpClient>,
    application_id: Id<ApplicationMarker>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    match event {
        Event::InteractionCreate(interaction) => {
            interaction::handle_interaction_create(*interaction, http, application_id).await
        }
        Event::Ready(ready) => ready::handle_ready(*ready).await,
        _ => Ok(()),
    }
}
