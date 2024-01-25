use config::File;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub token: String,
    pub ephemeral_replies: bool,
    pub mc_server_ip: String,
    pub dev_guild: Option<u64>,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .set_default("ephemeral_replies", "true")?
            .add_source(File::with_name("config.toml").required(false))
            .add_source(File::with_name("config.dev.toml").required(false))
            .add_source(config::Environment::with_prefix("FF").prefix_separator("_"))
            .build()?;

        config.try_deserialize()
    }
}
