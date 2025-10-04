use std::fs::File;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    std::fs::create_dir_all("logs").expect("Failed to create logs directory");
    let log_file = File::create("logs/app.log").expect("Failed to create log file");

    let file_subscriber = fmt::layer().with_writer(log_file).with_ansi(false);
    let stdout_subscriber = fmt::layer().with_ansi(true);

    let level = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .expect("Failed to parse log level from environment");

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(stdout_subscriber)
        .with(level)
        .init();
}
