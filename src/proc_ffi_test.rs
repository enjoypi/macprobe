use super::*;
use crate::support_test::cfg;

const MISSING: i32 = 999_999;

fn own() -> i32 {
    std::process::id() as i32
}

#[test]
fn responsible_of_self_is_positive() {
    assert!(responsible(&cfg().lineage, own()).unwrap() > 0);
}

#[test]
fn responsible_of_missing_pid_fails() {
    assert_eq!(
        responsible(&cfg().lineage, MISSING),
        Err(ProbeError::ResponsibleFailed)
    );
}

#[test]
fn responsible_library_missing() {
    let mut c = cfg().lineage;
    c.library = "/nonexistent".into();
    assert_eq!(responsible(&c, own()), Err(ProbeError::LoadFailed));
}

#[test]
fn responsible_symbol_missing() {
    let mut c = cfg().lineage;
    c.symbol = "NoSuchSymbol".into();
    assert_eq!(responsible(&c, own()), Err(ProbeError::SymbolMissing));
}

#[test]
fn parent_of_self_matches_std() {
    let expected = std::os::unix::process::parent_id() as i32;
    assert_eq!(parent(&cfg().lineage, own()), Ok(expected));
}

#[test]
fn parent_of_missing_pid_fails() {
    assert_eq!(
        parent(&cfg().lineage, MISSING),
        Err(ProbeError::ProcInfoFailed)
    );
}

#[test]
fn path_of_self_is_current_exe() {
    let exe = std::env::current_exe().unwrap();
    assert_eq!(path(&cfg().lineage, own()).unwrap(), exe.to_str().unwrap());
}

#[test]
fn path_of_missing_pid_fails() {
    assert_eq!(path(&cfg().lineage, MISSING), Err(ProbeError::PathFailed));
}

#[test]
fn parent_of_root_owned_launchd_is_zero() {
    assert_eq!(parent(&cfg().lineage, 1), Ok(0));
}

#[test]
fn priority_of_self_in_range() {
    let p = priority(&cfg().priority, own()).unwrap();
    assert!((0..=127).contains(&p));
}

#[test]
fn priority_of_missing_pid_fails() {
    assert_eq!(
        priority(&cfg().priority, MISSING),
        Err(ProbeError::ProcInfoFailed)
    );
}

#[test]
fn priority_with_full_buffer_is_truncated() {
    let mut c = cfg().priority;
    c.max_threads = 1;
    assert_eq!(priority(&c, own()), Err(ProbeError::ThreadsTruncated));
}

#[test]
fn priority_without_readable_thread_fails() {
    let mut c = cfg().priority;
    c.threadinfo_flavor = 0;
    assert_eq!(priority(&c, own()), Err(ProbeError::ProcInfoFailed));
}
