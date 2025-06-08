/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

mod args;
mod configutil;
mod error;
mod logutil;
mod proxy;
mod socks5;

use log::debug;
use std::sync::Arc;
use tokio::runtime::Builder;

use crate::args::Args;
use crate::configutil::Config;
use crate::error::Errors;
use crate::logutil::loginit;
use crate::socks5::start_server;

fn main() -> Result<(), Errors> {
    let args = Args::parse_args();
    let config = Config::load(&args)?;
    let config = Arc::new(config);

    loginit(&config)?;
    debug!("Config loaded: {:?}", config);

    debug!("Init tokio runtime...");
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { start_server(config).await })
}
