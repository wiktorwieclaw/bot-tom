use config::{Config, ConfigError, File, FileFormat};

#[derive(Clone, serde::Deserialize)]
pub struct Configuration {
    pub discord: DiscordConfiguration,
    pub cenzo_papa: CenzoPapaConfiguration,
}

#[derive(Clone, serde::Deserialize)]
pub struct DiscordConfiguration {
    pub token: String,
    pub guild_id: u64,
}

#[derive(Clone, serde::Deserialize)]
pub struct CenzoPapaConfiguration {
    pub channel_id: u64,
    pub songs: Vec<String>,
}

#[tracing::instrument("Reading configuration", skip_all)]
pub fn read_configuration() -> Result<Configuration, ConfigError> {
    Config::builder()
        .add_source(File::new("configuration/base", FileFormat::Toml))
        .add_source(File::new("configuration/local", FileFormat::Toml))
        .build()?
        .try_deserialize()
}
