//! Logging setup module.

pub fn init_logging() {
    tracing_subscriber::FmtSubscriber::builder()
        // To enable logging for ALL modules, uncomment the following line and comment
        // out the with_env_filter line.
        // To set the log level to debug for ONLY this module, change the with_env_filter
        // line to debug.
        .with_max_level(tracing::Level::INFO)
        // .with_env_filter("iron_docxide=info,iron_docxide_tests=info")
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .init();
}
