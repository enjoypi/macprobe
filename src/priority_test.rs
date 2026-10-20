use super::*;
use crate::support_test::{capture, cfg, ctx};
use tracing_subscriber::filter::LevelFilter;

const MISSING: i32 = 999_999;

fn own() -> i32 {
    std::process::id() as i32
}

fn ranked(pid: i32, priority: Result<i32, ProbeError>) -> Ranked {
    Ranked {
        proc: Proc {
            pid,
            path: Some(format!("/p{pid}")),
        },
        priority,
    }
}

fn tree(extra: Option<Ranked>) -> Tree {
    Tree {
        chain: vec![
            ranked(9, Ok(31)),
            ranked(1, Err(ProbeError::ProcInfoFailed)),
        ],
        extra,
    }
}

#[test]
fn render_joins_chain() {
    let c = cfg();
    assert_eq!(
        render(&tree(None), &c.priority, &c.messages.unknown, " < "),
        format!("9 /p9=31 < 1 /p1={}", c.messages.unknown)
    );
}

#[test]
fn render_appends_extra() {
    let c = cfg();
    let t = tree(Some(ranked(5, Ok(47))));
    assert!(render(&t, &c.priority, "?", " < ").ends_with("1 /p1=? | 5 /p5=47"));
}

#[test]
fn own_takes_chain_head() {
    assert_eq!(own_value(&tree(None), "?"), "31");
    let empty = Tree {
        chain: vec![],
        extra: None,
    };
    assert_eq!(own_value(&empty, "?"), "?");
}

#[test]
fn measure_self_logs_priority() {
    let c = cfg();
    let mut out = None;
    let logs = capture(LevelFilter::DEBUG, || {
        out = Some(measure(&c.lineage, &c.priority, &ctx(), own(), Some(own())))
    });
    let t = out.unwrap();
    assert_eq!(t.chain[0].proc.pid, own());
    assert!(t.chain[0].priority.is_ok());
    assert_eq!(t.extra, None);
    assert_eq!(logs[0]["feature"], c.priority.id);
    assert_eq!(logs[0]["action"], "priority");
    let req = logs[0]["req"].as_str().unwrap();
    assert!(req.starts_with(&format!("proc_pidinfo(6,5) [{}", own())));
}

#[test]
fn measure_lists_owner_off_chain() {
    let c = cfg();
    let mut out = None;
    capture(LevelFilter::INFO, || {
        out = Some(measure(
            &c.lineage,
            &c.priority,
            &ctx(),
            MISSING,
            Some(MISSING - 1),
        ))
    });
    let missing = |pid| Ranked {
        proc: Proc { pid, path: None },
        priority: Err(ProbeError::ProcInfoFailed),
    };
    let t = out.unwrap();
    assert_eq!(t.chain, vec![missing(MISSING)]);
    assert_eq!(t.extra, Some(missing(MISSING - 1)));
}

#[test]
fn inspector_reports_info() {
    let c = cfg();
    let i = PriorityInspector {
        lineage: &c.lineage,
        cfg: &c.priority,
        messages: &c.messages,
    };
    let logs = capture(LevelFilter::INFO, || i.inspect(&ctx()));
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0]["feature"], c.priority.id);
    assert_eq!(logs[0]["action"], "inspect");
}
