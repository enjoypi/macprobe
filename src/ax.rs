use crate::config::AxConfig;
use crate::log::{self, Ctx};
use crate::probe::{Outcome, Probe, Status};

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn AXIsProcessTrusted() -> u8;
}

pub fn trusted() -> u8 {
    unsafe { AXIsProcessTrusted() }
}

pub fn classify(raw: u8) -> Status {
    if raw == 0 {
        Status::Denied
    } else {
        Status::Granted
    }
}

pub struct AxProbe<'a> {
    pub cfg: &'a AxConfig,
}

impl Probe for AxProbe<'_> {
    fn check(&self, ctx: &Ctx) -> Outcome {
        let cfg = self.cfg;
        let raw = trusted();
        let detail = raw.to_string();
        log::debug(ctx, &cfg.id, "query", "AXIsProcessTrusted", &detail);
        Outcome {
            id: cfg.id.clone(),
            name: cfg.name.clone(),
            status: classify(raw),
            detail,
        }
    }
}

#[cfg(test)]
#[path = "ax_test.rs"]
mod tests;
