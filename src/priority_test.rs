use super::*;
use crate::lineage::{Lineage, LineageInspector, Tracer};
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

fn bare(pid: i32) -> Proc {
    Proc { pid, path: None }
}

fn measured(procs: Vec<Proc>, responsible: Proc) -> (Tree, Vec<serde_json::Value>) {
    let c = cfg();
    let l = Lineage {
        responsible: Ok(responsible),
        procs,
    };
    let mut out = None;
    let logs = capture(LevelFilter::DEBUG, || {
        out = Some(measure(&c.priority, &ctx(), &l))
    });
    (out.unwrap(), logs)
}

#[test]
fn measure_self_logs_priority() {
    let c = cfg();
    let (t, logs) = measured(vec![bare(own())], bare(own()));
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
    let (t, _) = measured(vec![bare(MISSING)], bare(MISSING - 1));
    let missing = |pid| Ranked {
        proc: bare(pid),
        priority: Err(ProbeError::ProcInfoFailed),
    };
    assert_eq!(t.chain, vec![missing(MISSING)]);
    assert_eq!(t.extra, Some(missing(MISSING - 1)));
}

#[test]
fn inspectors_share_one_trace() {
    let c = cfg();
    let t = Tracer::new(&c.lineage);
    let l = LineageInspector {
        tracer: &t,
        messages: &c.messages,
    };
    let p = PriorityInspector {
        tracer: &t,
        cfg: &c.priority,
        messages: &c.messages,
    };
    let logs = capture(LevelFilter::DEBUG, || {
        l.inspect(&ctx());
        p.inspect(&ctx());
    });
    let actions: Vec<&str> = logs.iter().map(|e| e["action"].as_str().unwrap()).collect();
    assert_eq!(actions, ["responsible", "inspect", "priority", "inspect"]);
    assert_eq!(logs[3]["feature"], c.priority.id);
}
