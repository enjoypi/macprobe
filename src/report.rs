use crate::config::{LineageConfig, Messages};
use crate::lineage::Lineage;
use crate::log::Ctx;
use crate::probe::{Outcome, Status};

fn label(m: &Messages, status: Status) -> &str {
    match status {
        Status::Granted => &m.granted,
        Status::Denied => &m.denied,
        Status::Unknown => &m.unknown,
    }
}

pub fn outcome(ctx: &Ctx, m: &Messages, o: &Outcome) {
    tracing::info!(
        request_id = %ctx.request_id,
        user = %ctx.user,
        host = %ctx.host,
        feature = %o.id,
        action = "check",
        status = o.status.as_str(),
        resp = %o.detail,
        "{}{}{}",
        o.name,
        m.separator,
        label(m, o.status)
    );
}

pub fn lineage(ctx: &Ctx, m: &Messages, cfg: &LineageConfig, l: &Lineage) {
    let (value, resp) = match &l.responsible {
        Ok(p) => (p.label(), p.label()),
        Err(e) => (m.unknown.clone(), format!("{e:?}")),
    };
    tracing::info!(
        request_id = %ctx.request_id,
        user = %ctx.user,
        host = %ctx.host,
        feature = %cfg.id,
        action = "inspect",
        resp = %resp,
        chain = %l.chain,
        "{}{}{}",
        cfg.name,
        m.separator,
        value
    );
}

pub fn usage(ctx: &Ctx, m: &Messages, args: &[String]) {
    tracing::error!(
        request_id = %ctx.request_id,
        user = %ctx.user,
        feature = "cli",
        action = "parse",
        req = %args.join(" "),
        "{}",
        m.usage
    );
}

pub fn version(ctx: &Ctx, name: &str, version: &str) {
    tracing::info!(
        request_id = %ctx.request_id,
        user = %ctx.user,
        feature = "cli",
        action = "version",
        resp = version,
        "{} {}",
        name,
        version
    );
}

#[cfg(test)]
#[path = "report_test.rs"]
mod tests;
