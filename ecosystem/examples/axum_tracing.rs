use std::path::PathBuf;

use ecosystem::telemetry::{get_subscriber, init_subscriber};
use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
use tracing::info;

fn main() {
    let log_dir = PathBuf::from("logs");
    std::fs::create_dir_all(&log_dir).expect("Failed to create logs directory");

    // 初始化文件滚动写入器（10MB切割，保留10个备份）
    let log_writer = FileRotate::new(
        "logs/app.log",
        AppendCount::new(5),
        ContentLimit::Bytes(1024 * 1024 * 10),
        Compression::None,
        #[cfg(unix)]
        Some(0o440),
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(log_writer);

    let subscriber = get_subscriber("axum_tracing".into(), "info".into(), non_blocking);

    init_subscriber(subscriber);

    info!("Hello, world!");
}
