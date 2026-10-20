use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub api_host: String,
    pub api_port: u16,
    pub web_base_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub session_secret: String,
    pub ai_provider: String,
    pub ai_base_url: Option<String>,
    pub ai_api_key: Option<String>,
    pub ai_default_model: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, config_crate::ConfigError> {
        let cfg = config_crate::Config::builder()
            .add_source(config_crate::Environment::default())
            .build()?;
        cfg.try_deserialize()
    }
}
