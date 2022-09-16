mod commands;
mod configuration;
mod event_handler;
mod startup;

use anyhow::Context;
use shuttle_service::SecretStore;
use sqlx::PgPool;

#[shuttle_service::main]
async fn serenity(#[shared::Postgres] pool: PgPool) -> shuttle_service::ShuttleSerenity {
    let configuration =
        configuration::read_configuration().context("Failed to read configuration")?;
    // Get the discord token set in `Secrets.toml` from the shared Postgres database
    let token = pool
        .get_secret("DISCORD_TOKEN")
        .await
        .context("Failed to get secret from database")?;
    let client = startup::setup_client(configuration, &token)
        .await
        .context("Failed to setup client")?;
    Ok(client)
}
