use std::fs;

use rand::{Rng, rngs::StdRng, SeedableRng};
use songbird::input::Input;

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
    
    // TODO: Error handling
    let states = guild.voice_states.get(&ctx.author().id).unwrap();

    // TODO: Error handling
    let vc = states.channel_id.unwrap();
    
    let manager = songbird::get(ctx.serenity_context()).await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler = manager.join(guild.id, vc).await.0;
    
    let mut tracks = vec![];
    for entry in fs::read_dir("./media/")? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let split = name.split('.').collect::<Vec<&str>>();
        let end = split[split.len() - 1];
        if end != "mp3" { continue; }
        tracks.push(name);
    };

    println!("{:?}", tracks);

    let mut rng = StdRng::from_entropy();
    let t_id: usize = rng.gen_range(0..tracks.len());
    println!("{}", t_id);
    let track = &tracks[t_id];

    println!("{}", track);

    let source = songbird::ffmpeg(format!("./media/{}", track)).await?;
    handler.lock().await.play_source(source);

    ctx.say("connecting").await?;

    Ok(())
}
