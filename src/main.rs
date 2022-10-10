use anyhow::Context;
use bot_tom::{configuration, startup, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = telemetry::setup_subscriber();
    telemetry::init_subscriber(subscriber).context("Failed to init tracing subscriber")?;

    let configuration =
        configuration::read_configuration().context("Failed to read configuration")?;

    let mut client = startup::setup_client(configuration)
        .await
        .context("Failed to setup client")?;

    client.start().await.context("Failed to start client")
}
