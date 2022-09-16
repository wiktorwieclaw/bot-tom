use crate::configuration::Configuration;
use anyhow::Context;
use delay_timer::prelude::*;
use rand::seq::SliceRandom;
use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::{Context as SerenityContext, EventHandler},
};
use songbird::id::ChannelId;
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
    async fn ready(&self, ctx: SerenityContext, _ready: Ready) {
        let ctx = Arc::new(ctx);
        let configuration = Arc::new(self.configuration.clone());

        DelayTimerBuilder::default()
            .build()
            .insert_task(
                TaskBuilder::default()
                    .set_task_id(1)
                    .set_frequency_once_by_cron_str("* 37 21 * * * *")
                    .spawn_async_routine(move || {
                        let ctx = Arc::clone(&ctx);
                        let configuration = Arc::clone(&configuration);
                        async {
                            cenzo_papa(ctx, configuration)
                                .await
                                .expect("Cenzo papa failed")
                        }
                    })
                    .expect("Failed to build barka task"),
            )
            .expect("Failed to insert barka task");
    }
}

async fn cenzo_papa(
    ctx: Arc<SerenityContext>,
    configuration: Arc<Configuration>,
) -> Result<(), anyhow::Error> {
    // TODO: support larger number of guilds
    let guild_id = ctx.cache.guilds()[0];
    // TODO: Find the channel with the most people
    let channel_id = ChannelId(configuration.cenzo_papa.channel_id);

    let songbird = songbird::get(&ctx)
        .await
        .context("Failed to initialize songbird client")?;
    let (call, _) = songbird.join(guild_id, channel_id).await;
    let mut call = call.lock().await;

    let audio_url = configuration
        .cenzo_papa
        .songs
        .choose(&mut rand::thread_rng())
        .context("The list of cenzo_papa songs is empty")?;
    let audio_source = songbird::ytdl(audio_url)
        .await
        .context("Failed to create streamed audio source")?;
    call.play_source(audio_source);
    Ok(())
}
