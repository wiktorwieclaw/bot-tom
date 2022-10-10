use crate::{
    command::general::GENERAL_GROUP,
    configuration::Configuration,
    event_handler::{self, BotEventHandler},
};
use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};
use songbird::SerenityInit;

#[tracing::instrument("Setting up client", skip_all)]
pub async fn setup_client(configuration: Configuration) -> Result<Client, serenity::Error> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix('!'))
        .before(event_handler::before)
        .after(event_handler::after)
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(&configuration.discord.token, intents)
        .event_handler(BotEventHandler::new(configuration))
        .framework(framework)
        .register_songbird()
        .await
}
