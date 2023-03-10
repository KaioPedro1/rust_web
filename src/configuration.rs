// Lê o arquivo .toml na pasta raiz e deserializa para o formato das struct abaixo
use config::Config;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
    pub redis: RedisSettings,
    pub jwt: Jwt,
}
#[derive(serde::Deserialize)]
pub struct Jwt {
    pub expiration: i64,
    pub secret_key: String,
}
#[derive(serde::Deserialize)]
pub struct RedisSettings {
    pub redis_url: String,
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
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_local_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()
        .unwrap();

    settings.try_deserialize()
}
