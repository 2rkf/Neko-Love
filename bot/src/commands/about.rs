use std::time::{SystemTime, UNIX_EPOCH};
use twilight_http::Client as HttpClient;
use twilight_model::id::marker::InteractionMarker;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::InteractionContextType,
    },
    channel::message::{embed::EmbedField, Embed},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{marker::ApplicationMarker, Id},
    oauth::ApplicationIntegrationType,
    util::Timestamp,
};

pub async fn handle(
    http: &HttpClient,
    interaction_id: Id<InteractionMarker>,
    interaction_token: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = Timestamp::from_secs(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards(?)")
            .as_secs() as i64,
    )?;
    let embed = Embed {
        title: Some("About me!".to_string()),
        description: Some("I'm a Discord bot for **Neko-Love**.".to_string()),
        color: Some(0xe4ffcb),
        fields: Vec::new(),
        author: None,
        footer: None,
        image: None,
        thumbnail: None,
        video: None,
        provider: None,
        timestamp: Some(timestamp),
        url: None,
        kind: "rich".to_string(),
    };

    http.interaction(interaction_id.cast())
        .create_response(
            interaction_id,
            interaction_token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    embeds: Some(vec![embed]),
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
        name: "about".to_string(),
        description: "Get information about me.".to_string(),
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
