use config;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.toml",
            config::FileFormat::Toml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
