use crate::config::{LineageConfig, Messages};
use crate::error::ProbeError;
use crate::log::{self, Ctx};
use crate::probe::Inspector;
use crate::proc_ffi::{parent, path, responsible};
use crate::report;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proc {
    pub pid: i32,
    pub path: Option<String>,
}

impl Proc {
    pub fn of(cfg: &LineageConfig, pid: i32) -> Proc {
        Proc {
            pid,
            path: path(cfg, pid).ok(),
        }
    }

    pub fn label(&self) -> String {
        match &self.path {
            Some(p) => format!("{} {}", self.pid, p),
            None => self.pid.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lineage {
    pub responsible: Result<Proc, ProbeError>,
    pub chain: String,
}

pub fn ancestors(cfg: &LineageConfig, start: i32) -> Vec<Proc> {
    let mut procs = Vec::new();
    let mut pid = start;
    while procs.len() < cfg.max_depth {
        procs.push(Proc::of(cfg, pid));
        if pid == cfg.root_pid {
            break;
        }
        match parent(cfg, pid) {
            Ok(p) => pid = p,
            Err(_) => break,
        }
    }
    procs
}

pub fn chain(procs: &[Proc], separator: &str) -> String {
    let labels: Vec<String> = procs.iter().map(Proc::label).collect();
    labels.join(separator)
}

pub fn trace(cfg: &LineageConfig, ctx: &Ctx, pid: i32) -> Lineage {
    let raw = responsible(cfg, pid);
    let req = format!("{}({pid})", cfg.symbol);
    log::debug(ctx, &cfg.id, "responsible", &req, &format!("{raw:?}"));
    Lineage {
        responsible: raw.map(|r| Proc::of(cfg, r)),
        chain: chain(&ancestors(cfg, pid), &cfg.separator),
    }
}

pub struct LineageInspector<'a> {
    pub cfg: &'a LineageConfig,
    pub messages: &'a Messages,
}

impl Inspector for LineageInspector<'_> {
    fn inspect(&self, ctx: &Ctx) {
        let l = trace(self.cfg, ctx, std::process::id() as i32);
        report::lineage(ctx, self.messages, self.cfg, &l);
    }
}

#[cfg(test)]
#[path = "lineage_test.rs"]
mod tests;
