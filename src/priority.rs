use crate::config::{LineageConfig, Messages, PriorityConfig};
use crate::error::ProbeError;
use crate::lineage::{Proc, ancestors};
use crate::log::{self, Ctx};
use crate::probe::Inspector;
use crate::proc_ffi::{priority, responsible};
use crate::report;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ranked {
    pub proc: Proc,
    pub priority: Result<i32, ProbeError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    pub chain: Vec<Ranked>,
    pub extra: Option<Ranked>,
}

fn value(r: &Ranked, unknown: &str) -> String {
    r.priority.map_or(unknown.to_string(), |p| p.to_string())
}

fn label(r: &Ranked, cfg: &PriorityConfig, unknown: &str) -> String {
    format!("{}{}{}", r.proc.label(), cfg.assign, value(r, unknown))
}

pub fn render(t: &Tree, cfg: &PriorityConfig, unknown: &str, separator: &str) -> String {
    let labels: Vec<String> = t.chain.iter().map(|r| label(r, cfg, unknown)).collect();
    let mut text = labels.join(separator);
    if let Some(e) = &t.extra {
        text.push_str(&cfg.extra);
        text.push_str(&label(e, cfg, unknown));
    }
    text
}

pub fn own_value(t: &Tree, unknown: &str) -> String {
    t.chain
        .first()
        .map_or(unknown.to_string(), |r| value(r, unknown))
}

pub fn measure(
    lc: &LineageConfig,
    cfg: &PriorityConfig,
    ctx: &Ctx,
    pid: i32,
    owner: Option<i32>,
) -> Tree {
    let procs = ancestors(lc, pid);
    let extra = owner
        .filter(|r| !procs.iter().any(|p| p.pid == *r))
        .map(|r| Proc::of(lc, r));
    let rank = |proc: Proc| Ranked {
        priority: priority(cfg, proc.pid),
        proc,
    };
    let t = Tree {
        chain: procs.into_iter().map(rank).collect(),
        extra: extra.map(rank),
    };
    let all: Vec<&Ranked> = t.chain.iter().chain(t.extra.iter()).collect();
    let pids: Vec<i32> = all.iter().map(|r| r.proc.pid).collect();
    let raw: Vec<Result<i32, ProbeError>> = all.iter().map(|r| r.priority).collect();
    let req = format!(
        "proc_pidinfo({},{}) {pids:?}",
        cfg.listthreads_flavor, cfg.threadinfo_flavor
    );
    log::debug(ctx, &cfg.id, "priority", &req, &format!("{raw:?}"));
    t
}

pub struct PriorityInspector<'a> {
    pub lineage: &'a LineageConfig,
    pub cfg: &'a PriorityConfig,
    pub messages: &'a Messages,
}

impl Inspector for PriorityInspector<'_> {
    fn inspect(&self, ctx: &Ctx) {
        let pid = std::process::id() as i32;
        let owner = responsible(self.lineage, pid).ok();
        let t = measure(self.lineage, self.cfg, ctx, pid, owner);
        let unknown = &self.messages.unknown;
        let tree = render(&t, self.cfg, unknown, &self.lineage.separator);
        report::priority(ctx, self.messages, self.cfg, &own_value(&t, unknown), &tree);
    }
}

#[cfg(test)]
#[path = "priority_test.rs"]
mod tests;
