use super::*;
use crate::support_test::{capture, cfg, ctx};
use tracing_subscriber::filter::LevelFilter;

fn echo(text: &str) -> DevToolsConfig {
    let mut c = cfg().devtools;
    c.command = "/bin/echo".into();
    c.args = vec![text.into()];
    c
}

#[test]
fn parse_enabled() {
    let c = cfg().devtools;
    assert_eq!(parse(&format!("{}\n", c.enabled), &c), Status::Granted);
}

#[test]
fn parse_disabled() {
    let c = cfg().devtools;
    assert_eq!(parse(&format!(" {} ", c.disabled), &c), Status::Denied);
}

#[test]
fn parse_other_is_unknown() {
    assert_eq!(parse("garbage", &cfg().devtools), Status::Unknown);
}

#[test]
fn exec_spawn_failure() {
    let mut c = cfg().devtools;
    c.command = "/nonexistent".into();
    assert_eq!(exec(&c), Err(ProbeError::SpawnFailed));
}

#[test]
fn probe_nonzero_exit_is_unknown() {
    let mut c = cfg().devtools;
    c.command = "/usr/bin/false".into();
    c.args = vec![];
    let mut out = None;
    capture(LevelFilter::INFO, || {
        out = Some(DevToolsProbe { cfg: &c }.check(&ctx()))
    });
    assert_eq!(out.unwrap().status, Status::Unknown);
}

#[test]
fn probe_granted_and_logs() {
    let c = echo(&cfg().devtools.enabled);
    let mut out = None;
    let logs = capture(LevelFilter::DEBUG, || {
        out = Some(DevToolsProbe { cfg: &c }.check(&ctx()))
    });
    let out = out.unwrap();
    assert_eq!(out.status, Status::Granted);
    assert_eq!(out.id, c.id);
    assert_eq!(out.name, c.name);
    assert_eq!(out.detail, c.enabled);
    assert_eq!(logs[0]["action"], "exec");
    assert_eq!(logs[0]["req"], format!("/bin/echo {}", c.enabled));
}

#[test]
fn probe_error_is_unknown() {
    let mut c = cfg().devtools;
    c.command = "/nonexistent".into();
    let mut out = None;
    capture(LevelFilter::INFO, || {
        out = Some(DevToolsProbe { cfg: &c }.check(&ctx()))
    });
    let out = out.unwrap();
    assert_eq!(out.status, Status::Unknown);
    assert_eq!(out.detail, "SpawnFailed");
}
