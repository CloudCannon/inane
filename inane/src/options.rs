use anyhow::Result;
use clap::Parser;
use std::env;
use twelf::config;

use crate::logging::{LogLevel, Logger};

#[config]
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct InaneInboundConfig {
    #[clap(long, short, help = "The location of your built static website")]
    #[clap(required = false)]
    #[serde(default = "defaults::default_message")]
    pub message: String,
}

mod defaults {
    pub fn default_message() -> String {
        "Hello World".into()
    }
}

// The configuration object used internally
#[derive(Debug)]
pub struct InaneOptions {
    pub message: String,
    pub logger: Logger,
    pub version: &'static str,
}

impl InaneOptions {
    pub fn load(config: InaneInboundConfig) -> Result<Self> {
        Ok(Self {
            message: config.message,
            logger: Logger::new(LogLevel::Verbose),
            version: env!("CARGO_PKG_VERSION"),
        })
    }
}
