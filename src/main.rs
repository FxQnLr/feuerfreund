use std::env;

use poise::{serenity_prelude as serenity, Framework};
use songbird::SerenityInit;
use time::UtcOffset;
use tracing::{Level, info, debug};
use tracing_subscriber::{fmt::time::OffsetTime, FmtSubscriber};

use feuerfreund::{commands, Data};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let offset_var: i8 = env::var("TIME_OFFSET")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();
    let offset = UtcOffset::from_hms(offset_var, 0, 0).expect("invalid offset");
    let time_format =
        time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let timer = OffsetTime::new(offset, time_format);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_timer(timer)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("starting Feuerfreund v{}", env!("CARGO_PKG_VERSION"));
    
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::help::help(),
            commands::connect::connect(),
            commands::disconnect::disconnect(),
        ],
        pre_command: |ctx| {
            Box::pin(async move {
                debug!("executing command {}", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                info!("executed command {}", ctx.command().qualified_name);
            })
        },
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                // TODO: handle events individually
                debug!("got an event in event handler: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .token(env::var("TOKEN").expect("missing DISCORD_TOKEN"))
        .client_settings(|client| client.register_songbird())
        .intents(serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::GUILD_VOICE_STATES)
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                register(ctx, framework).await;
                info!("registered commands");
                info!("logged in as {}#{}", ready.user.name, ready.user.discriminator);
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}

#[cfg(debug_assertions)]
async fn register(ctx: &::serenity::prelude::Context, framework: &Framework<Data, Box<dyn std::error::Error + Send + Sync>>) {
    use tracing::warn;
    use serenity::GuildId;

    warn!("register commands locally");
    let guild_id = env::var("DEV_GUILD").expect("can't register without guild id");
    poise::builtins::register_in_guild(
        ctx,
        &framework.options().commands,
        GuildId::from(guild_id.parse::<u64>().unwrap())
    ).await.unwrap();
}

#[cfg(not(debug_assertions))]
async fn register(ctx: &::serenity::prelude::Context, framework: &Framework<Data, Box<dyn std::error::Error + Send + Sync>>) {
    info!("register commands globally");
    poise::builtins::register_globally(ctx, &framework.options().commands).await.unwrap();
}
