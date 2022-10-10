use std::sync::Arc;

use anyhow::Context;
use delay_timer::prelude::{Task, TaskBuilder, TaskError};
use rand::seq::SliceRandom;
use serenity::{model::id::ChannelId, prelude::Context as SerenityContext};
use url::Url;

use crate::{audio, configuration::Configuration};

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
            async { cenzo(ctx, configuration).await.expect("Cenzo papa failed") }
        })
}

#[tracing::instrument("Playing cenzo", skip_all)]
async fn cenzo(
    ctx: Arc<SerenityContext>,
    configuration: Arc<Configuration>,
) -> Result<(), anyhow::Error> {
    // TODO: support larger number of guilds
    let guild_id = ctx.cache.guilds()[0];

    // TODO: Find the most populated channel
    let channel_id = ChannelId(configuration.cenzo_papa.channel_id);

    let raw_url = configuration
        .cenzo_papa
        .songs
        .choose(&mut rand::thread_rng())
        .context("The list of cenzo songs is empty")?;
    let url = Url::parse(raw_url).context("Invalid url")?;

    audio::join_and_play(&ctx, guild_id, channel_id, &url)
        .await
        .context("Failed to play audio")?;
    Ok(())
}
