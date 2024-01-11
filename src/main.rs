use tracing::{info, error};
use tracing_subscriber::{filter::LevelFilter, EnvFilter, prelude::*, fmt::{self, time::UtcTime}};

fn main() -> color_eyre::Result<()> {

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

    info!("hallo");

    error!("hallo");

    Ok(())
}

