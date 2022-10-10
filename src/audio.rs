use anyhow::Context;
use serenity::{
    model::id::{ChannelId, GuildId},
    prelude::Context as SerenityContext,
};
use songbird::tracks::TrackHandle;
use url::Url;

#[tracing::instrument("Playing audio", skip(ctx, guild_id, channel_id))]
pub async fn join_and_play(
    ctx: &SerenityContext,
    guild_id: GuildId,
    channel_id: ChannelId,
    url: &Url,
) -> anyhow::Result<TrackHandle> {
    let songbird = songbird::get(ctx)
        .await
        .context("Failed to initialize Songbird client")?;

    let (call, result) = songbird.join(guild_id, channel_id).await;
    result.context("Failed to join channel")?;

    let mut call = call.lock().await;
    let audio_source = songbird::ytdl(&url)
        .await
        .context("Failed to create an streamed audio source")?;

    Ok(call.play_source(audio_source))
}
