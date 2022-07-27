use std::env;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub connection_string: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Http {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configs {
    pub debug: bool,
    pub database: Database,
    pub http: Http,
}

impl Configs {
    pub fn new() -> Result<Self, ConfigError> {
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let configs = Config::builder()
            .add_source(File::with_name("src/config/default"))
            .add_source(File::with_name(&format!("src/config/{}", environment)).required(false))
            .add_source(File::with_name("src/config/local").required(false))
            .build()?;

        configs.try_deserialize()
    }
}
