use anyhow::Context;
use tracing::Subscriber;
use tracing_log::LogTracer;
use tracing_subscriber::{filter::LevelFilter, fmt::format::FmtSpan, prelude::*, EnvFilter};

pub fn setup_subscriber() -> impl Subscriber + Sync + Send {
    let fmt_layer =
        tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let target_filter_layer = tracing_subscriber::filter::Targets::new()
        .with_default(LevelFilter::TRACE)
        .with_target("serenity", LevelFilter::WARN)
        .with_target("songbird", LevelFilter::WARN)
        .with_target("delay_timer", LevelFilter::WARN);
    tracing_subscriber::registry()
        .with(target_filter_layer)
        .with(env_filter)
        .with(fmt_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) -> anyhow::Result<()> {
    LogTracer::init().context("Failed to init LogTracer")?;
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set subscriber as global defaul")
}
