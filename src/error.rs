/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Config parse error: {0}")]
    ConfigError(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
}
