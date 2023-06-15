//! Logging setup module.
extern crate env_logger;
extern crate log;
use env_logger::fmt::Color;
use log::Level;
use std::io::Write;
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
    if let Some(env) = env {
        tracing_subscriber::registry()
            .with(filter)
            .with(EnvFilter::from_str(env).unwrap())
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

pub fn init_env_logging(colors: bool, level: log::LevelFilter, env: Option<&str>) {
    env_logger::Builder::new()
        .format(move |buf, record| {
            let level = record.level();
            let mut style = buf.style();
            if colors {
                match record.level() {
                    Level::Error => style.set_color(Color::Red),
                    Level::Warn => style.set_color(Color::Yellow),
                    Level::Info => style.set_color(Color::Green),
                    Level::Debug => style.set_color(Color::Blue),
                    Level::Trace => style.set_color(Color::Cyan),
                };
            }

            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                style.value(level),
                record.args()
            )
        })
        .filter(env, level)
        .init();
}
