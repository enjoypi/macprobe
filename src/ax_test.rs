use super::*;
use crate::support_test::{capture, cfg, ctx};
use tracing_subscriber::filter::LevelFilter;

#[test]
fn classify_raw_values() {
    assert_eq!(classify(0), Status::Denied);
    assert_eq!(classify(1), Status::Granted);
    assert_eq!(classify(2), Status::Granted);
}

#[test]
fn probe_real_and_logs() {
    let c = cfg().ax;
    let mut out = None;
    let logs = capture(LevelFilter::DEBUG, || {
        out = Some(AxProbe { cfg: &c }.check(&ctx()))
    });
    let out = out.unwrap();
    assert_eq!(out.status, classify(out.detail.parse().unwrap()));
    assert_eq!(out.id, c.id);
    assert_eq!(out.name, c.name);
    assert_eq!(logs[0]["action"], "query");
    assert_eq!(logs[0]["req"], "AXIsProcessTrusted");
}
