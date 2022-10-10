use crate::{configuration::Configuration, scheduler};
use serenity::{
    async_trait,
    framework::standard::{macros::hook, CommandResult},
    model::{gateway::Ready, prelude::Message},
    prelude::{Context as SerenityContext, EventHandler},
};
use std::sync::Arc;

pub struct BotEventHandler {
    configuration: Configuration,
}

impl BotEventHandler {
    pub fn new(configuration: Configuration) -> Self {
        BotEventHandler { configuration }
    }
}

#[async_trait]
impl EventHandler for BotEventHandler {
    #[tracing::instrument("Setting up event handler", skip_all)]
    async fn ready(&self, ctx: SerenityContext, _bot_info: Ready) {
        let ctx = Arc::new(ctx);
        let configuration = Arc::new(self.configuration.clone());

        scheduler::spawn_scheduler(Arc::clone(&configuration), Arc::clone(&ctx))
            .expect("Failed to spawn scheduler");
    }
}

#[hook]
#[tracing::instrument(skip(_ctx, _msg))]
pub async fn before(_ctx: &SerenityContext, _msg: &Message, _command_name: &str) -> bool {
    true
}

#[hook]
#[tracing::instrument(skip(_ctx, _msg))]
pub async fn after(
    ctx: &SerenityContext,
    msg: &Message,
    _command_name: &str,
    command_result: CommandResult,
) {
    if let Err(e) = command_result {
        msg.channel_id
            .say(&ctx.http, format!("{e}"))
            .await
            .expect("Failed to send error message");
    };
}
