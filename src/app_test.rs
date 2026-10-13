use super::*;
use crate::support_test::{capture, cfg, ctx};
use serde_json::Value;
use std::cell::Cell;
use tracing_subscriber::filter::LevelFilter;

struct Fake {
    status: Status,
    calls: Cell<u32>,
}

impl Fake {
    fn new(status: Status) -> Self {
        Fake {
            status,
            calls: Cell::new(0),
        }
    }
}

impl Probe for Fake {
    fn check(&self, _ctx: &Ctx) -> Outcome {
        self.calls.set(self.calls.get() + 1);
        Outcome {
            id: "fake".into(),
            name: "假".into(),
            status: self.status,
            detail: String::new(),
        }
    }
}

impl Inspector for Fake {
    fn inspect(&self, _ctx: &Ctx) {
        self.calls.set(self.calls.get() + 1);
    }
}

fn exit_of(statuses: &[Status]) -> (u8, Vec<Value>) {
    let fakes: Vec<Fake> = statuses.iter().map(|s| Fake::new(*s)).collect();
    let probes: Vec<&dyn Probe> = fakes.iter().map(|f| f as &dyn Probe).collect();
    let mut code = 0;
    let logs = capture(LevelFilter::INFO, || {
        code = run(&[], &cfg(), &ctx(), &[], &probes)
    });
    (code, logs)
}
#[test]
fn all_granted_exits_granted() {
    assert_eq!(
        exit_of(&[Status::Granted, Status::Granted]).0,
        cfg().exit.granted
    );
}

#[test]
fn denied_exits_denied() {
    assert_eq!(
        exit_of(&[Status::Granted, Status::Denied]).0,
        cfg().exit.denied
    );
}

#[test]
fn unknown_exits_unknown() {
    assert_eq!(
        exit_of(&[Status::Denied, Status::Unknown]).0,
        cfg().exit.unknown
    );
}

#[test]
fn logs_only_each_outcome() {
    let (_, logs) = exit_of(&[Status::Granted, Status::Denied]);
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0]["status"], "granted");
    assert_eq!(logs[1]["status"], "denied");
}

#[test]
fn args_are_usage_error_without_probing() {
    let fake = Fake::new(Status::Granted);
    let mut code = 0;
    let logs = capture(LevelFilter::INFO, || {
        code = run(&["-x".into()], &cfg(), &ctx(), &[&fake], &[&fake])
    });
    assert_eq!(code, cfg().exit.unknown);
    assert_eq!(logs.len(), 1);
    assert_eq!(fake.calls.get(), 0);
}

#[test]
fn check_runs_inspectors_without_affecting_exit() {
    let inspector = Fake::new(Status::Unknown);
    let probe = Fake::new(Status::Granted);
    let mut code = 0;
    capture(LevelFilter::INFO, || {
        code = run(&[], &cfg(), &ctx(), &[&inspector], &[&probe])
    });
    assert_eq!(code, cfg().exit.granted);
    assert_eq!((inspector.calls.get(), probe.calls.get()), (1, 1));
}

fn run_args(args: &[&str]) -> (u8, Vec<Value>, u32) {
    let fake = Fake::new(Status::Granted);
    let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
    let mut code = 0;
    let logs = capture(LevelFilter::INFO, || {
        code = run(&args, &cfg(), &ctx(), &[&fake], &[&fake])
    });
    (code, logs, fake.calls.get())
}

#[test]
fn version_flags_print_version_without_probing() {
    for flag in ["--version", "-V"] {
        let (code, logs, calls) = run_args(&[flag]);
        assert_eq!(code, cfg().exit.ok);
        assert_eq!(calls, 0);
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0]["resp"], env!("CARGO_PKG_VERSION"));
    }
}

#[test]
fn version_flag_with_extra_args_is_usage_error() {
    let (code, logs, calls) = run_args(&["--version", "x"]);
    assert_eq!(code, cfg().exit.unknown);
    assert_eq!(calls, 0);
    assert_eq!(logs[0]["level"], "ERROR");
}
