use super::*;
use crate::support_test::{capture, cfg, ctx};
use tracing_subscriber::filter::LevelFilter;

const MISSING: i32 = 999_999;

fn own() -> i32 {
    std::process::id() as i32
}

fn proc(pid: i32, path: Option<&str>) -> Proc {
    Proc {
        pid,
        path: path.map(String::from),
    }
}

#[test]
fn label_with_and_without_path() {
    assert_eq!(proc(7, Some("/bin/zsh")).label(), "7 /bin/zsh");
    assert_eq!(proc(7, None).label(), "7");
}

#[test]
fn chain_joins_labels() {
    let procs = [proc(9, Some("/a")), proc(1, None)];
    assert_eq!(chain(&procs, " < "), "9 /a < 1");
}

#[test]
fn ancestors_start_at_self_then_parent() {
    let procs = ancestors(&cfg().lineage, own());
    assert_eq!(procs[0].pid, own());
    assert!(procs[0].path.is_some());
    assert_eq!(procs[1].pid, std::os::unix::process::parent_id() as i32);
}

#[test]
fn ancestors_stop_at_root() {
    let c = cfg().lineage;
    let procs = ancestors(&c, c.root_pid);
    assert_eq!(procs.len(), 1);
    assert_eq!(procs[0].pid, c.root_pid);
}

#[test]
fn ancestors_stop_on_error() {
    assert_eq!(
        ancestors(&cfg().lineage, MISSING),
        vec![proc(MISSING, None)]
    );
}

#[test]
fn ancestors_respect_max_depth() {
    let mut c = cfg().lineage;
    c.max_depth = 1;
    assert_eq!(ancestors(&c, own()).len(), 1);
}

#[test]
fn trace_self_finds_responsible() {
    let c = cfg().lineage;
    let mut out = None;
    let logs = capture(LevelFilter::DEBUG, || out = Some(trace(&c, &ctx(), own())));
    let out = out.unwrap();
    assert!(out.responsible.unwrap().pid > 0);
    assert!(out.chain.starts_with(&own().to_string()));
    assert_eq!(logs[0]["feature"], c.id);
    assert_eq!(logs[0]["action"], "responsible");
    assert_eq!(logs[0]["req"], format!("{}({})", c.symbol, own()));
}

#[test]
fn trace_missing_pid_is_error() {
    let c = cfg().lineage;
    let mut out = None;
    capture(LevelFilter::INFO, || out = Some(trace(&c, &ctx(), MISSING)));
    assert_eq!(out.unwrap().responsible, Err(ProbeError::ResponsibleFailed));
}

#[test]
fn inspector_reports_info() {
    let c = cfg();
    let i = LineageInspector {
        cfg: &c.lineage,
        messages: &c.messages,
    };
    let logs = capture(LevelFilter::INFO, || i.inspect(&ctx()));
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0]["feature"], c.lineage.id);
    assert_eq!(logs[0]["action"], "inspect");
}
