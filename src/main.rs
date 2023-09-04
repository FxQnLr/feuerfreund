use std::env;

use time::UtcOffset;
use tracing::{Level, info};
use tracing_subscriber::{fmt::time::OffsetTime, FmtSubscriber};

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
}
