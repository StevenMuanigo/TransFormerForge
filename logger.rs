use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use crate::config::AppConfig;

pub fn init_logger(config: &AppConfig) {
    let log_level = &config.monitoring.log_level;
    let log_file = &config.monitoring.log_file;
    
    // File appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        log_file.parent().unwrap_or_else(|| std::path::Path::new("./")),
        log_file.file_name().unwrap_or_else(|| std::ffi::OsStr::new("app.log")),
    );
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Create layers
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .json();
    
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .pretty();
    
    // Initialize subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(log_level)
        }))
        .with(file_layer)
        .with(stdout_layer)
        .init();
    
    tracing::info!("Logger initialized with level: {}", log_level);
}
