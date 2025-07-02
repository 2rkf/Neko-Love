use std::time::Instant;
use twilight_http::Client as HttpClient;
use twilight_model::id::marker::InteractionMarker;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::InteractionContextType,
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{marker::ApplicationMarker, Id},
    oauth::ApplicationIntegrationType,
};

pub async fn handle(
    http: &HttpClient,
    interaction_id: Id<InteractionMarker>,
    interaction_token: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start_time = Instant::now();

    http.interaction(interaction_id.cast())
        .create_response(
            interaction_id,
            interaction_token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    content: Some(format!("Pong! {}ms", start_time.elapsed().as_millis())),
                    flags: None,
                    ..Default::default()
                }),
            },
        )
        .await?;

    Ok(())
}

pub fn register(application_id: Id<ApplicationMarker>) -> Command {
    Command {
        application_id: Some(application_id),
        name: "ping".to_string(),
        description: "Get a pong response.".to_string(),
        options: Vec::new(),
        default_member_permissions: None,
        guild_id: None,
        id: Id::new_checked(0),
        kind: CommandType::ChatInput,
        name_localizations: None,
        dm_permission: Some(true),
        description_localizations: None,
        contexts: Some(vec![
            InteractionContextType::Guild,
            InteractionContextType::BotDm,
            InteractionContextType::PrivateChannel,
        ]),
        integration_types: Some(vec![
            ApplicationIntegrationType::GuildInstall,
            ApplicationIntegrationType::UserInstall,
        ]),
        nsfw: Some(false),
        version: Id::new(1),
    }
}
