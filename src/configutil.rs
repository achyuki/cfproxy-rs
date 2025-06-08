/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

use crate::{args::Args, error::Errors};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Config {
    pub cfhost: String,
    pub cfip: String,
    pub token: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub passwd: String,
    pub log: PathBuf,
    pub loglevel: String,
}

impl Config {
    pub fn get_loglevel(&self) -> LevelFilter {
        LevelFilter::from_str(&self.loglevel).unwrap_or(LevelFilter::Info)
    }

    pub fn load(args: &Args) -> Result<Self, Errors> {
        let mut config = Config::default();
        if let Some(path) = &args.config {
            // log not initialized
            // info!("Loading config from: {}", path.display());
            config = fs::read_to_string(path)
                .map_err(|e| Errors::IoError(e))
                .and_then(|content| {
                    serde_json::from_str(&content).map_err(|e| Errors::ConfigError(e))
                })?;
        }

        if let Some(v) = &args.cfhost {
            config.cfhost = v.clone();
        }
        if let Some(v) = &args.cfip {
            config.cfip = v.clone();
        }
        if let Some(v) = &args.token {
            config.token = v.clone();
        }
        if let Some(v) = &args.host {
            config.host = v.clone();
        }
        if let Some(v) = args.port {
            config.port = v;
        }
        if let Some(v) = &args.user {
            config.user = v.clone();
        }
        if let Some(v) = &args.passwd {
            config.passwd = v.clone();
        }
        if let Some(v) = &args.log {
            config.log = v.clone();
        }
        if let Some(v) = &args.loglevel {
            config.loglevel = v.clone();
        }

        if config.cfhost.is_empty() {
            println!("Missing the required config 'cfhost'!");
            std::process::exit(1);
        }
        Ok(config)
    }
}

// Default configuration
impl Default for Config {
    fn default() -> Self {
        Self {
            cfhost: String::new(),
            cfip: "104.16.0.0".into(), // CFIP
            token: String::new(),
            host: "127.0.0.1".into(),
            port: 4514,
            user: String::new(),
            passwd: String::new(),
            log: PathBuf::new(),
            loglevel: "info".into(),
        }
    }
}
