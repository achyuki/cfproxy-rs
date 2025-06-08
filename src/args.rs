/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cfproxy-rs")]
#[command(author = "achyuki")]
#[command(version = "0.1.0")]
#[command(about = "Socks5 proxy server based on Cloudflare Workers/Pages.", long_about = None)]
pub struct Args {
    /// Config file path
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Cloudflare workers/pages domain [Required]
    #[arg(long)]
    pub cfhost: Option<String>,

    /// Cloudflare IP [default: 104.16.0.0]
    #[arg(long)]
    pub cfip: Option<String>,

    /// Authentication token
    #[arg(long)]
    pub token: Option<String>,

    /// SOCKS5 bind address [default: 127.0.0.1]
    #[arg(long)]
    pub host: Option<String>,

    /// SOCKS5 bind port [default: 4514]
    #[arg(long)]
    pub port: Option<u16>,

    /// SOCKS5 username
    #[arg(long)]
    pub user: Option<String>,

    /// SOCKS5 password
    #[arg(long)]
    pub passwd: Option<String>,

    /// Log saving path
    #[arg(long, value_name = "PATH")]
    pub log: Option<PathBuf>,

    /// Log level (error/warn/info/debug/trace) [default: info]
    #[arg(long)]
    pub loglevel: Option<String>,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
