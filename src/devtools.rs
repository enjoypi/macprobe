use crate::config::DevToolsConfig;
use crate::error::ProbeError;
use crate::log::{self, Ctx};
use crate::probe::{Outcome, Probe, Status};
use std::process::Command;

pub fn exec(cfg: &DevToolsConfig) -> Result<String, ProbeError> {
    let out = Command::new(&cfg.command)
        .args(&cfg.args)
        .output()
        .map_err(|_| ProbeError::SpawnFailed)?;
    if !out.status.success() {
        return Err(ProbeError::ExitFailed);
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn parse(stdout: &str, cfg: &DevToolsConfig) -> Status {
    let text = stdout.trim();
    if text == cfg.enabled {
        Status::Granted
    } else if text == cfg.disabled {
        Status::Denied
    } else {
        Status::Unknown
    }
}

pub struct DevToolsProbe<'a> {
    pub cfg: &'a DevToolsConfig,
}

impl Probe for DevToolsProbe<'_> {
    fn check(&self, ctx: &Ctx) -> Outcome {
        let cfg = self.cfg;
        let req = format!("{} {}", cfg.command, cfg.args.join(" "));
        let (status, detail) = match exec(cfg) {
            Ok(out) => (parse(&out, cfg), out.trim().to_string()),
            Err(e) => (Status::Unknown, format!("{e:?}")),
        };
        log::debug(ctx, &cfg.id, "exec", &req, &detail);
        Outcome {
            id: cfg.id.clone(),
            name: cfg.name.clone(),
            status,
            detail,
        }
    }
}

#[cfg(test)]
#[path = "devtools_test.rs"]
mod tests;
