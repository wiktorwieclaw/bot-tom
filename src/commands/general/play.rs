use anyhow::Context;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::Context as SerenityContext,
};

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &SerenityContext, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id)
        .context("Not in a voice channel")?;

    let songbird = songbird::get(ctx)
        .await
        .context("Songbird Voice client placed in at initialisation")?;
    let (call, result) = songbird.join(guild_id, channel_id).await;
    result.context("Failed to join channel")?;

    let raw_url: String = args
        .single()
        .context("Must provide a URL to a video or audio")?;
    let url = url::Url::parse(&raw_url).context("Invalid URL")?;

    let mut call = call.lock().await;
    let audio_source = songbird::ytdl(&url)
        .await
        .context("Failed to create a streamed audio source")?;
    call.play_source(audio_source);
    msg.channel_id
        .say(&ctx.http, "Playing song")
        .await
        .context("Error sending message")?;
    Ok(())
}
