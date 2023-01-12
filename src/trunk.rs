//! Logging setup module.
use std::str::FromStr;
use tracing_subscriber::{filter, fmt, prelude::*, reload, EnvFilter, Registry};

extern crate tracing_subscriber;

pub fn init_logging() {
    tracing_subscriber::FmtSubscriber::builder()
        // To enable logging for ALL modules, uncomment the following line and comment
        // out the with_env_filter line.
        // To set the log level to debug for ONLY this module, change the with_env_filter
        // line to debug.
        .with_max_level(tracing::Level::INFO)
        // .with_env_filter("example=info")
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}

/// Initialize logging that can be modified.
/// Useful for if you want to enable a verbose mode that prints debug logs.
pub fn init_mod_logging(
    level: filter::LevelFilter,
    env: Option<&str>,
) -> reload::Handle<filter::LevelFilter, Registry> {
    let filter = level;
    let (filter, reload_handle) = reload::Layer::new(filter);
    if env.is_none() {
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::Layer::default()
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(EnvFilter::from_str(env.unwrap()).unwrap())
            .with(
                fmt::Layer::default()
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    }

    reload_handle
}
