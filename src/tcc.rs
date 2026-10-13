use crate::config::TccConfig;
use crate::log::{self, Ctx};
use crate::probe::{Outcome, Probe, Status};
use crate::tcc_ffi::preflight;

pub fn classify(code: i32, cfg: &TccConfig) -> Status {
    if cfg.granted.contains(&code) {
        Status::Granted
    } else if cfg.denied.contains(&code) {
        Status::Denied
    } else {
        Status::Unknown
    }
}

pub struct TccProbe<'a> {
    pub cfg: &'a TccConfig,
}

impl Probe for TccProbe<'_> {
    fn check(&self, ctx: &Ctx) -> Outcome {
        let cfg = self.cfg;
        let (status, detail) = match preflight(cfg) {
            Ok(code) => (classify(code, cfg), code.to_string()),
            Err(e) => (Status::Unknown, format!("{e:?}")),
        };
        log::debug(ctx, &cfg.id, "preflight", &cfg.service, &detail);
        Outcome {
            id: cfg.id.clone(),
            name: cfg.name.clone(),
            status,
            detail,
        }
    }
}

#[cfg(test)]
#[path = "tcc_test.rs"]
mod tests;
