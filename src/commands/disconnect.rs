use crate::{Error, Context};

#[poise::command(
    slash_command,
    guild_only,
    description_localized("en-US", "disconnect from voice channel"),
    description_localized("en-GB", "disconnect from voice channel")
)]
pub async fn disconnect(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            // TODO: human readable error
            ctx.say(e.to_string()).await?;
        }

        ctx.say("Left voice channel").await?;
    } else {
        ctx.say("Not in a voice channel").await?;
    }

    Ok(())
}
