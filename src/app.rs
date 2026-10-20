use crate::ax::AxProbe;
use crate::config::{self, Config, ExitConfig};
use crate::devtools::DevToolsProbe;
use crate::fda::FdaProbe;
use crate::lineage::LineageInspector;
use crate::log::{self, Ctx};
use crate::priority::PriorityInspector;
use crate::probe::{Inspector, Outcome, Probe, Status, overall};
use crate::report;
use crate::tcc::TccProbe;

fn exit_code(status: Status, exit: &ExitConfig) -> u8 {
    match status {
        Status::Granted => exit.granted,
        Status::Denied => exit.denied,
        Status::Unknown => exit.unknown,
    }
}

pub fn run(
    args: &[String],
    cfg: &Config,
    ctx: &Ctx,
    inspectors: &[&dyn Inspector],
    probes: &[&dyn Probe],
) -> u8 {
    match args {
        [] => {
            for i in inspectors {
                i.inspect(ctx);
            }
            check(cfg, ctx, probes)
        }
        [flag] if cfg.cli.version_flags.contains(flag) => {
            report::version(ctx, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            cfg.exit.ok
        }
        _ => {
            report::usage(ctx, &cfg.messages, args);
            cfg.exit.unknown
        }
    }
}

fn check(cfg: &Config, ctx: &Ctx, probes: &[&dyn Probe]) -> u8 {
    let outcomes: Vec<Outcome> = probes.iter().map(|p| p.check(ctx)).collect();
    for o in &outcomes {
        report::outcome(ctx, &cfg.messages, o);
    }
    exit_code(overall(&outcomes), &cfg.exit)
}

pub fn main_entry() -> u8 {
    let cfg = config::embedded();
    log::init(&cfg.log);
    let ctx = Ctx::generate();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let lineage = LineageInspector {
        cfg: &cfg.lineage,
        messages: &cfg.messages,
    };
    let priority = PriorityInspector {
        lineage: &cfg.lineage,
        cfg: &cfg.priority,
        messages: &cfg.messages,
    };
    let tcc = TccProbe { cfg: &cfg.tcc };
    let fda = FdaProbe { cfg: &cfg.fda };
    let ax = AxProbe { cfg: &cfg.ax };
    let devtools = DevToolsProbe { cfg: &cfg.devtools };
    run(
        &args,
        &cfg,
        &ctx,
        &[&lineage, &priority],
        &[&tcc, &fda, &ax, &devtools],
    )
}

#[cfg(test)]
#[path = "app_test.rs"]
mod tests;
