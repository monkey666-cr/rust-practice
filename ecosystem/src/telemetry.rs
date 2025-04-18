use std::time::Duration;

use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, Tracer},
    Resource,
};
use tracing::{subscriber::set_global_default, Event, Subscriber};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{
        self,
        format::FmtSpan,
        time::{ChronoLocal, FormatTime},
        FormatEvent, FormatFields, MakeWriter,
    },
    layer::SubscriberExt,
    registry::LookupSpan,
    EnvFilter,
};

pub fn get_subscriber<Sink>(
    _name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S".into());
    // let format = fmt::format()
    //     .with_level(true) // 显示日志等级
    //     .with_thread_ids(true) // 显示线程ID
    //     .with_line_number(true)
    //     .with_target(false) // 隐藏模块路径
    //     .with_timer(timer); // 自定义时间格式

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // 初始化OpenTelemetry
    let tracer = init_optl_tracer().unwrap();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // 创建日志订阅者
    let subscriber = tracing_subscriber::registry()
        .with(env_filter) // 从RUST_LOG环境变量读取配置
        .with(
            fmt::Layer::new()
                .with_writer(sink) // 输出到滚动文件
                .with_ansi(false) // 禁用ANSI颜色
                .event_format(CustomFormatter), // 应用自定义格式
        )
        .with(telemetry);

    #[cfg(debug_assertions)]
    let subscriber = subscriber.with(
        fmt::Layer::new()
            .with_writer(std::io::stdout)
            .with_span_events(FmtSpan::CLOSE)
            .pretty()
            .event_format(CustomFormatter), // 应用自定义格式
    );

    return subscriber;
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

fn init_optl_tracer() -> anyhow::Result<Tracer> {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .with_timeout(Duration::from_secs(3))
        .build()?;

    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(otlp_exporter, opentelemetry_sdk::runtime::Tokio)
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_max_events_per_span(16)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "axum-tracing",
        )]))
        .build();

    let tracer = tracer_provider.tracer("axum-tracing");

    Ok(tracer)
}

struct CustomFormatter;

impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: fmt::format::Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        // 自定义时间格式
        let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S".into());
        write!(writer, "{}", "[")?;
        timer.format_time(&mut writer)?;
        write!(writer, "{}", "] ")?;

        // 显示日志级别
        let level = *event.metadata().level();
        write!(writer, "[{}] ", level)?;

        // 显示线程 ID
        let thread_id = std::thread::current().id();
        write!(writer, "[{:?}] ", thread_id)?;

        let thread_name = std::thread::current()
            .name()
            .unwrap_or("unknown")
            .to_string();
        write!(writer, "[{}] ", thread_name)?;

        // 显示目标模块
        let target = event.metadata().target();
        write!(writer, "[{}] ", target)?;

        // 显示日志消息
        ctx.format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
