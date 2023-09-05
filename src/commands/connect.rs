use crate::{Error, Context};

#[poise::command(
    slash_command,
    guild_only,
    description_localized("en-US", "connect to voice channel"),
    description_localized("en-GB", "connect to voice channel")
)]
pub async fn connect(
    ctx: Context<'_>,
) -> Result<(), Error> {
    // TODO: Error handling
    let guild = ctx.guild().unwrap();
    
    // let member = guild.member(ctx, ctx.author().id).await?;

    // TODO: Error handling
    let states = guild.voice_states.get(&ctx.author().id).unwrap();

    // TODO: Error handling
    let vc = states.channel_id.unwrap();
    
    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild.id, vc).await.0;

    let source = songbird::ffmpeg("./media/fire.mp3").await?;

    _handler.lock().await.play_source(source);

    ctx.say("connecting").await?;

    Ok(())
}
