use super::*;
use crate::support_test::{capture, cfg, ctx};
use tracing_subscriber::filter::LevelFilter;

#[test]
fn classify_codes() {
    let c = cfg().tcc;
    assert_eq!(classify(0, &c), Status::Granted);
    assert_eq!(classify(1, &c), Status::Denied);
    assert_eq!(classify(2, &c), Status::Denied);
    assert_eq!(classify(99, &c), Status::Unknown);
}

#[test]
fn probe_real_returns_code_and_logs() {
    let c = cfg().tcc;
    let mut out = None;
    let logs = capture(LevelFilter::DEBUG, || {
        out = Some(TccProbe { cfg: &c }.check(&ctx()))
    });
    let out = out.unwrap();
    assert_eq!(out.id, c.id);
    assert_eq!(out.name, c.name);
    assert_eq!(out.status, classify(out.detail.parse().unwrap(), &c));
    assert_eq!(logs[0]["action"], "preflight");
    assert_eq!(logs[0]["req"], c.service);
}

#[test]
fn probe_error_is_unknown() {
    let mut c = cfg().tcc;
    c.framework = "/nonexistent".into();
    let mut out = None;
    capture(LevelFilter::INFO, || {
        out = Some(TccProbe { cfg: &c }.check(&ctx()))
    });
    let out = out.unwrap();
    assert_eq!(out.status, Status::Unknown);
    assert_eq!(out.detail, "LoadFailed");
}
