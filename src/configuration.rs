use config::{Config, ConfigError, File, FileFormat};

#[derive(Clone, serde::Deserialize)]
pub struct Configuration {
    pub cenzo_papa: CenzoPapaConfiguration,
}

#[derive(Clone, serde::Deserialize)]
pub struct CenzoPapaConfiguration {
    pub channel_id: u64,
    pub songs: Vec<String>,
}

pub fn read_configuration() -> Result<Configuration, ConfigError> {
    Config::builder()
        .add_source(File::new("configuration/base", FileFormat::Toml))
        .build()?
        .try_deserialize()
}
