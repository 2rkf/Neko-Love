# Getting Started

::: warning
Seria is currently under heavy development. Please refrain from using it in the meantime.
:::

## Requirements

Seria supports a MSRV of **Rust 1.76 or later**.

## Installation

Create a new Cargo project for your bot using this following command:

```bash
cargo new your_bot
```

After creating a new project, enter the directory and install Seria:

```bash
cargo add seria
cargo add tokio --features macros --features rt-multi-thread
```

## Creating a Revolt Bot

Here's a concise guide on how to create a Revolt bot:

1. Log in to Revolt
    - Go to [Revolt's official website](https://revolt.chat/) and log in to your account.
2. Create a Bot Account
    - Click on your own settings.
    - Go to **Settings → My Bots → Create a Bot**
    - Give it a name and click **Create**.
3. Obtain the Bot Token
    - After creation, you will see a **Token** section under your bot.
    - Click on the **Copy** button.

## Ping Pong Example

Add the following code inside your `src/main.rs` file.

```rs
use seria::{
    client::{SeriaClient, SeriaClientBuilder},
    models::{GatewayEvent, MessageSend},
    SeriaResult,
    StreamExt,
};
use std::{pin::pin, sync::Arc};
use tracing::{error, warn};

async fn handle_event(event: GatewayEvent, client: Arc<SeriaClient>) {
    match event {
        GatewayEvent::Ready => {
            if let Ok(user) = client.http.get_self().await {
                println!("{}#{} is Ready!", user.username, user.discriminator);
            }
        }
        GatewayEvent::Message(message) => {
            let content = message.content.to_string();

            if content == "!ping" {
                let payload = MessageSend {
                    content: "Pong!".to_string(),
                    ..Default::default()
                };

                if let Err(e) = client.http.send_message(&message.channel, payload).await {
                    error!("Failed to send message: {}", e);
                }
            }
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() -> SeriaResult<()> {
    tracing_subscriber::fmt::init();

    let token = "REVOLT_TOKEN";

    let mut client = SeriaClientBuilder::new().
        token(token)
        .build()?;

    client.connect().await?;

    let client = Arc::new(client);

    let mut event_stream = pin!(client.gateway.clone());

    while let Some(item) = event_stream.next().await {
        match item {
            Ok(event) => {
                let client = Arc::clone(&client);
                tokio::spawn(async move {
                    handle_event(event, client).await;
                });
            }
            Err(e) => {
                warn!(error = ?e, "Failed to receive event");
            }
        }
    }

    Ok(())
}
```

Once you added the code, replace `REVOLT_TOKEN` with your actual bot token and run your bot with `cargo run`
