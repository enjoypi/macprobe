use crate::config::LogConfig;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::Subscriber;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::MakeWriter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ctx {
    pub request_id: String,
    pub user: String,
    pub host: String,
}

impl Ctx {
    pub fn from_env(cfg: &LogConfig) -> Ctx {
        let read = |name: &str| std::env::var(name).unwrap_or(cfg.unset.clone());
        Ctx {
            request_id: request_id(),
            user: read(&cfg.user_env),
            host: read(&cfg.host_env),
        }
    }
}

pub fn request_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}-{:x}", std::process::id(), nanos)
}

pub fn level(raw: Option<&str>, default: &str) -> LevelFilter {
    let fallback = default.parse().expect("log.default_level");
    raw.and_then(|s| s.parse().ok()).unwrap_or(fallback)
}

pub fn subscriber<W>(level: LevelFilter, writer: W) -> impl Subscriber + Send + Sync
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_target(false)
        .with_max_level(level)
        .with_writer(writer)
        .finish()
}

pub fn init(cfg: &LogConfig) {
    let raw = std::env::var(&cfg.level_env).ok();
    let sub = subscriber(level(raw.as_deref(), &cfg.default_level), std::io::stdout);
    tracing::subscriber::set_global_default(sub).expect("log init");
}

pub fn debug(ctx: &Ctx, feature: &str, action: &str, req: &str, resp: &str) {
    tracing::debug!(
        request_id = %ctx.request_id,
        user = %ctx.user,
        feature,
        action,
        req,
        resp
    );
}

#[cfg(test)]
#[path = "log_test.rs"]
mod tests;
