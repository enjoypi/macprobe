use super::*;
use crate::support_test::{capture, cfg, ctx};
use tracing_subscriber::filter::LevelFilter;

#[test]
fn open_existing_file() {
    assert_eq!(
        open(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml")),
        Ok(())
    );
}

#[test]
fn open_missing_file() {
    assert_eq!(open("/nonexistent"), Err(Some(2)));
}

#[test]
fn open_invalid_path_has_no_errno() {
    assert_eq!(open("a\0b"), Err(None));
}

#[test]
fn classify_results() {
    let c = cfg().fda;
    assert_eq!(classify(Ok(()), &c), Status::Granted);
    assert_eq!(classify(Err(Some(1)), &c), Status::Denied);
    assert_eq!(classify(Err(Some(2)), &c), Status::Unknown);
    assert_eq!(classify(Err(None), &c), Status::Unknown);
}

#[test]
fn probe_real_path_and_logs() {
    let c = cfg().fda;
    let mut out = None;
    let logs = capture(LevelFilter::DEBUG, || {
        out = Some(FdaProbe { cfg: &c }.check(&ctx()))
    });
    let out = out.unwrap();
    assert_ne!(out.status, Status::Unknown);
    assert_eq!(out.id, c.id);
    assert_eq!(out.name, c.name);
    assert_eq!(logs[0]["action"], "open");
    assert_eq!(logs[0]["req"], c.path);
}

#[test]
fn probe_missing_path_is_unknown() {
    let mut c = cfg().fda;
    c.path = "/nonexistent".into();
    let mut out = None;
    capture(LevelFilter::INFO, || {
        out = Some(FdaProbe { cfg: &c }.check(&ctx()))
    });
    let out = out.unwrap();
    assert_eq!(out.status, Status::Unknown);
    assert_eq!(out.detail, "Err(Some(2))");
}
