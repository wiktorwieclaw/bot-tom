use crate::{
    commands::general::GENERAL_GROUP, configuration::Configuration, event_handler::BotEventHandler,
};
use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};
use songbird::SerenityInit;

pub async fn setup_client(
    configuration: Configuration,
    token: &str,
) -> Result<Client, serenity::Error> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix('!'))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(&token, intents)
        .event_handler(BotEventHandler::new(configuration))
        .framework(framework)
        .register_songbird()
        .await
}
