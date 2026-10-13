use super::*;
use crate::support_test::{capture, cfg, ctx};

#[test]
fn level_defaults_when_unset() {
    assert_eq!(level(None, "info"), LevelFilter::INFO);
}

#[test]
fn level_reads_valid_value() {
    assert_eq!(level(Some("debug"), "info"), LevelFilter::DEBUG);
}

#[test]
fn level_falls_back_on_invalid_value() {
    assert_eq!(level(Some("bogus"), "info"), LevelFilter::INFO);
}

#[test]
fn request_id_starts_with_pid() {
    assert!(request_id().starts_with(&format!("{:x}-", std::process::id())));
}

#[test]
fn ctx_reads_env_or_unset() {
    let mut c = cfg().log;
    c.user_env = "PATH".into();
    c.host_env = "MACPROBE_TEST_ABSENT_VAR".into();
    let x = Ctx::from_env(&c);
    assert_eq!(x.user, std::env::var("PATH").unwrap());
    assert_eq!(x.host, c.unset);
    assert!(!x.request_id.is_empty());
}

#[test]
fn debug_emits_structured_utc_json() {
    let logs = capture(LevelFilter::DEBUG, || {
        debug(&ctx(), "feat", "act", "q", "r")
    });
    assert_eq!(logs.len(), 1);
    let e = &logs[0];
    assert_eq!(e["level"], "DEBUG");
    assert_eq!(e["request_id"], "rid");
    assert_eq!(e["user"], "tester");
    assert_eq!(e["feature"], "feat");
    assert_eq!(e["action"], "act");
    assert_eq!(e["req"], "q");
    assert_eq!(e["resp"], "r");
    assert!(e["timestamp"].as_str().unwrap().ends_with('Z'));
}

#[test]
fn debug_hidden_at_info() {
    assert!(capture(LevelFilter::INFO, || debug(&ctx(), "f", "a", "q", "r")).is_empty());
}
