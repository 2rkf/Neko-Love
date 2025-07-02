use twilight_gateway::{ConfigBuilder, Intents, Shard, ShardId};
use twilight_model::gateway::{
    payload::outgoing::update_presence::UpdatePresencePayload,
    presence::{ActivityType, ClientStatus, MinimalActivity, Status},
};

pub fn create_shard(
    token: String,
) -> Result<Shard, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let intents = Intents::empty();
    let presence = UpdatePresencePayload {
        activities: vec![MinimalActivity {
            kind: ActivityType::Listening,
            name: "your heart ♡".into(),
            url: None,
        }
        .into()],
        afk: false,
        since: None,
        status: Status::Online,
    };

    let config = ConfigBuilder::new(token, intents)
        .presence(presence)
        .build();

    Ok(Shard::with_config(ShardId::ONE, config))
}
