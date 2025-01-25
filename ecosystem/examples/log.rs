use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
use std::path::PathBuf;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    // 创建日志目录（如果不存在）
    let log_dir = PathBuf::from("logs");
    std::fs::create_dir_all(&log_dir).expect("Failed to create logs directory");

    // 初始化文件滚动写入器（10MB切割，保留10个备份）
    let log_writer = FileRotate::new(
        "logs/app.log",
        AppendCount::new(5),
        // ContentLimit::Bytes(1024 * 1024 * 10),
        ContentLimit::Lines(50),
        Compression::None,
        #[cfg(unix)]
        Some(0o440),
    );

    // 配置日志格式：时间 线程ID 等级 消息
    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S".into());
    let format = fmt::format()
        .with_level(true) // 显示日志等级
        .with_thread_ids(true) // 显示线程ID
        .with_target(false) // 隐藏模块路径
        .with_timer(timer); // 自定义时间格式

    let (non_blocking, _guard) = tracing_appender::non_blocking(log_writer);

    // 创建日志订阅者
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()) // 从RUST_LOG环境变量读取配置
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking) // 输出到滚动文件
                .with_ansi(false) // 禁用ANSI颜色
                .event_format(format), // 应用自定义格式
        );

    // 设置全局日志订阅者
    subscriber.init();

    // 示例日志记录
    tracing::info!("System initialized");
    tracing::warn!("This is a warning message");
    tracing::error!("This is an error message");

    // 多线程测试
    let handles: Vec<_> = (0..4)
        .map(|i| {
            std::thread::spawn(move || {
                for j in 0..100 {
                    tracing::info!(thread = i, count = j, "Processing data");
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
