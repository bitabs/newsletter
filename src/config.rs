use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        // consume the values from our yaml file
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        // this will initiate the build
        .build()?;

    // try to convert the values into our settings type
    settings.try_deserialize::<Settings>()
}
