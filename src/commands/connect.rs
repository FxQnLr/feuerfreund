use std::fs;

use rand::{Rng, rngs::StdRng, SeedableRng};
use serenity::async_trait;
use songbird::{Event, TrackEvent, id::GuildId, EventHandler, EventContext, input::Input};
use tracing::{info, error};

use crate::{Error, Context};

struct TrackEndNotifier {
    guild_id: GuildId,
    context: serenity::prelude::Context,
}

#[async_trait]
impl EventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_) = ctx {
            info!("track ended, playing new one");
            let manager = songbird::get(&self.context).await
                .expect("Songbird Voice client placed in at initialisation.")
                .clone();

            let handler = manager.get(self.guild_id);
            if handler.is_none() {
                error!("Error no handler")
            }

            handler.unwrap().lock().await.play_source(get_source().await.unwrap());
            
        }

        None
    }
}

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
    

    let mut handle = handler.lock().await;

    handle.deafen(true).await?;

    handle.play_source(get_source().await?);

    handle.add_global_event(
        Event::Track(TrackEvent::End),
        TrackEndNotifier {
            guild_id: guild.id.into(),
            context: ctx.serenity_context().to_owned(),
        },
    );

    ctx.say("connected").await?;

    Ok(())
}


async fn get_source() -> Result<Input, songbird::input::error::Error> {
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

    songbird::ffmpeg(format!("./media/{}", track)).await
}