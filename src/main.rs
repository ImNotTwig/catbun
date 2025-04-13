use std::env;

mod twilight_types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("TOKEN")?;
    let http_client = twilight_types::new_twilight_http_client(token);
    let user = http_client.current_user().await?.model().await?;

    tracing::info!("Logged in as: {}!", user.id);

    Ok(())
}
