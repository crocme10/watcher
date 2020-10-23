use clap::ArgMatches;
use config::{Config, Environment, File};
use serde::Deserialize;
use snafu::ResultExt;
use std::env;
use std::path::PathBuf;

use super::error;

#[derive(Debug, Clone, Deserialize)]
pub struct Journal {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub testing: bool,
    pub mode: String,
    pub service: Service,
    pub journal: Journal,
}

// TODO Parameterize the config directory

impl Settings {
    pub fn new<'a, T: Into<Option<&'a ArgMatches<'a>>>>(_matches: T) -> Result<Self, error::Error> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))
            .context(error::ConfigError {
                msg: String::from("Could not merge default configuration"),
            })?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let mode = env::var("RUN_MODE").unwrap_or_else(|_| String::from("development"));
        s.merge(File::with_name(&format!("config/{}", mode)).required(true))
            .context(error::ConfigError {
                msg: format!("Could not merge {} configuration", mode),
            })?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))
            .context(error::ConfigError {
                msg: String::from("Could not merge local configuration"),
            })?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))
            .context(error::ConfigError {
                msg: String::from("Could not merge configuration from environment variables"),
            })?;

        // Now we take care of the database.url, which can be had from environment variables.
        let key = match mode.as_str() {
            "testing" => "ASSETS_TEST_PATH",
            _ => "ASSETS_PATH",
        };

        let assets_path = env::var(key).context(error::EnvVarError {
            msg: format!("Could not get env var {}", key),
        })?;

        s.set("service.path", assets_path)
            .context(error::ConfigError {
                msg: String::from("Could not set database url from environment variable"),
            })?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into().context(error::ConfigError {
            msg: String::from("Could not generate settings from configuration"),
        })
    }
}
