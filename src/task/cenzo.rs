use std::sync::Arc;

use anyhow::Context;
use delay_timer::prelude::{Task, TaskBuilder, TaskError};
use rand::seq::SliceRandom;
use serenity::prelude::Context as SerenityContext;
use songbird::id::ChannelId;

use crate::configuration::Configuration;

#[tracing::instrument("Building cenzo task", skip_all)]
pub fn build_cenzo(
    configuration: Arc<Configuration>,
    ctx: Arc<SerenityContext>,
) -> Result<Task, TaskError> {
    TaskBuilder::default()
        .set_task_id(1)
        .set_frequency_once_by_cron_str("* 37 21 * * * *")
        .spawn_async_routine(move || {
            let ctx = Arc::clone(&ctx);
            let configuration = Arc::clone(&configuration);
            async {
                cenzo(ctx, configuration)
                    .await
                    .expect("Cenzo papa failed")
            }
        })
}

#[tracing::instrument("Playing cenzo", skip_all)]
async fn cenzo(
    ctx: Arc<SerenityContext>,
    configuration: Arc<Configuration>,
) -> Result<(), anyhow::Error> {
    tracing::info!("Playing cenzo");
    // TODO: support larger number of guilds
    let guild_id = ctx.cache.guilds()[0];
    tracing::info!("Guild id - {guild_id}");
    // TODO: Find the channel with the most people
    let channel_id = ChannelId(configuration.cenzo_papa.channel_id);

    let audio_url = configuration
        .cenzo_papa
        .songs
        .choose(&mut rand::thread_rng())
        .context("The list of cenzo_papa songs is empty")?;

    let songbird = songbird::get(&ctx)
        .await
        .context("Failed to initialize songbird client")?;
    let (call, _) = songbird.join(guild_id, channel_id).await;
    let mut call = call.lock().await;

    let audio_source = songbird::ytdl(audio_url)
        .await
        .context("Failed to create streamed audio source")?;
    call.play_source(audio_source);
    Ok(())
}
