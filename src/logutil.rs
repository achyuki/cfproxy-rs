/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

use env_logger::{Builder, Target};
use log::info;
use std::fs::OpenOptions;

use crate::configutil::Config;
use crate::error::Errors;

pub fn loginit(config: &Config) -> Result<(), Errors> {
    let mut builder = Builder::new();
    let loglevel = config.get_loglevel();
    builder.filter_level(loglevel);

    if config.log.components().count() != 0 {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&config.log)?;

        builder.target(Target::Pipe(Box::new(file)));
        builder.init();
        info!(
            "Logging to file {} with level {}",
            config.log.display(),
            loglevel
        );
    } else {
        builder.target(Target::Stdout);
        builder.init();
        info!("Logging to stdout with level {}", loglevel);
    }

    Ok(())
}
