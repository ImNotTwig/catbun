use std::env;

use twilight_gateway::{Intents, ShardId};

mod twilight_types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("TOKEN")?;
    let http_client = twilight_types::new_twilight_http_client(token.clone());
    let user = http_client.current_user().await?.model().await?;

    let mut shard =
        twilight_types::new_twilight_gateway_client(ShardId::ONE, token, Intents::all());

    tracing::info!("Logged in as: {}!", user.name);

    Ok(())
}
