use crate::config::FdaConfig;
use crate::log::{self, Ctx};
use crate::probe::{Outcome, Probe, Status};
use std::fs::File;

pub fn open(path: &str) -> Result<(), Option<i32>> {
    File::open(path).map(drop).map_err(|e| e.raw_os_error())
}

pub fn classify(res: Result<(), Option<i32>>, cfg: &FdaConfig) -> Status {
    match res {
        Ok(()) => Status::Granted,
        Err(Some(n)) if cfg.denied_errno.contains(&n) => Status::Denied,
        Err(_) => Status::Unknown,
    }
}

pub struct FdaProbe<'a> {
    pub cfg: &'a FdaConfig,
}

impl Probe for FdaProbe<'_> {
    fn check(&self, ctx: &Ctx) -> Outcome {
        let cfg = self.cfg;
        let res = open(&cfg.path);
        let detail = format!("{res:?}");
        log::debug(ctx, &cfg.id, "open", &cfg.path, &detail);
        Outcome {
            id: cfg.id.clone(),
            name: cfg.name.clone(),
            status: classify(res, cfg),
            detail,
        }
    }
}

#[cfg(test)]
#[path = "fda_test.rs"]
mod tests;
