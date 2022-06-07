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

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .set_default("database.database_name", "newsletter")?
        .set_default("database.port", 5432)?
        .add_source(config::File::with_name("configuration"))
        .add_source(config::Environment::with_prefix("APP_"))
        .build()?;
    settings.try_deserialize()
}
