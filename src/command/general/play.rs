use anyhow::Context;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::Context as SerenityContext,
};
use url::Url;

use crate::audio;

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &SerenityContext, msg: &Message, mut args: Args) -> CommandResult {
    let _ = tracing::info_span!("Play command", author = msg.author.name).entered();

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id)
        .context("Not in a voice channel")?;

    let raw_url: String = args
        .single()
        .context("Must provide an URL to a video or audio")?;
    let url = Url::parse(&raw_url).context("Invalid url")?;

    audio::join_and_play(ctx, guild_id, channel_id, &url)
        .await
        .context("Failed to play audio")?;

    msg.channel_id
        .say(&ctx.http, "Playing song")
        .await
        .context("Error sending message")?;
    Ok(())
}
