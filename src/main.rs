use serenity::{prelude::{GatewayIntents, Context}, client::ClientBuilder, all::{GuildId, OnlineStatus}, gateway::ActivityData};
use tracing::{info, debug, warn};
use tracing_subscriber::{filter::LevelFilter, EnvFilter, prelude::*, fmt::{self, time::UtcTime}};
use poise::Framework;

use crate::config::Config;

mod commands;
mod config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Data {
    pub config: Config,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    color_eyre::install()?;

    let time_format = time::macros::format_description!("[hour]:[minute]:[second]"); 

    let file_appender = tracing_appender::rolling::never(".", "feuerfreund.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(fmt::layer()
            .with_writer(non_blocking)
        )
        .with(fmt::layer()
            .with_timer(UtcTime::new(time_format))
        )
        .with(EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
        )
        .try_init()?;

    let config = Config::load()?;
    let config_st = config.clone();

    info!("start feuerfreund v{VERSION}");

    let framework = poise::Framework::builder()
        .options(
            poise::FrameworkOptions {
                commands: vec![
                    commands::help(),
                    commands::minecraft(),
                ],
                pre_command: |ctx| {
                    Box::pin(async move {
                        debug!("executing command {} by {}", ctx.command().qualified_name, ctx.author().name);
                    })
                },
                ..Default::default()
            }
        )
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                if let Some(guild_id) = config_st.dev_guild {
                    register_locally(ctx, framework, guild_id).await?;
                } else {
                    register_globally(ctx, framework).await?;
                }
                info!("registered commands");
                info!("logged in as {}#{:?}", ready.user.name, ready.user.discriminator);
                let ctx_us = ctx.clone();
                let config_us = config_st.clone();
                std::thread::spawn(move || update_status(&ctx_us, &config_us));
                Ok(Data { config: config_st } )
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged();

    ClientBuilder::new(config.token, intents)
        .framework(framework)
        .await?
        .start()
        .await?;

    Ok(())
}

async fn register_locally(ctx: &Context, framework: &Framework<Data, Box<dyn std::error::Error + Send + Sync>>, guild_id: u64) -> Result<(), serenity::Error> {
    warn!("register commands locally");
    poise::builtins::register_in_guild(
        ctx,
        &framework.options().commands,
        GuildId::from(guild_id)
    ).await
}

async fn register_globally(ctx: &Context, framework: &Framework<Data, Box<dyn std::error::Error + Send + Sync>>) -> Result<(), serenity::Error> {
    info!("register commands globally");
    poise::builtins::register_globally(ctx, &framework.options().commands).await
}

fn update_status(ctx: &Context, config: &Config) {
    loop {
        let stats = mcsc_query::basic_stats(&config.mc_server_ip);
        // info!(?stats);
        if stats.is_ok() {
            ctx.set_presence(Some(ActivityData::custom("server online")), OnlineStatus::Online);
        } else {
            ctx.set_presence(Some(ActivityData::custom("server offline")), OnlineStatus::DoNotDisturb);
        };
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
