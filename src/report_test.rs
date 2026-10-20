use super::*;
use crate::error::ProbeError;
use crate::lineage::{Lineage, Proc};
use crate::support_test::{capture, cfg, ctx};
use tracing_subscriber::filter::LevelFilter;

fn sample(status: Status) -> Outcome {
    Outcome {
        id: "pid".into(),
        name: "名称".into(),
        status,
        detail: "d".into(),
    }
}

#[test]
fn outcome_is_info_json() {
    let m = cfg().messages;
    let logs = capture(LevelFilter::INFO, || {
        outcome(&ctx(), &m, &sample(Status::Granted))
    });
    let e = &logs[0];
    assert_eq!(e["level"], "INFO");
    assert_eq!(e["request_id"], "rid");
    assert!(e.get("user").is_none());
    assert!(e.get("host").is_none());
    assert_eq!(e["feature"], "pid");
    assert_eq!(e["action"], "check");
    assert_eq!(e["status"], "granted");
    assert_eq!(e["resp"], "d");
    assert_eq!(e["message"], format!("名称{}{}", m.separator, m.granted));
}

#[test]
fn outcome_labels_each_status() {
    let m = cfg().messages;
    for (s, label) in [
        (Status::Granted, &m.granted),
        (Status::Denied, &m.denied),
        (Status::Unknown, &m.unknown),
    ] {
        let logs = capture(LevelFilter::INFO, || outcome(&ctx(), &m, &sample(s)));
        assert_eq!(logs[0]["status"], s.as_str());
        assert_eq!(logs[0]["message"], format!("名称{}{}", m.separator, label));
    }
}

#[test]
fn usage_is_error() {
    let m = cfg().messages;
    let logs = capture(LevelFilter::INFO, || {
        usage(&ctx(), &m, &["-x".into(), "y".into()])
    });
    assert_eq!(logs[0]["level"], "ERROR");
    assert_eq!(logs[0]["req"], "-x y");
    assert_eq!(logs[0]["message"], m.usage);
}

#[test]
fn version_is_info() {
    let logs = capture(LevelFilter::INFO, || version(&ctx(), "macprobe", "9.9.9"));
    assert_eq!(logs[0]["level"], "INFO");
    assert_eq!(logs[0]["feature"], "cli");
    assert_eq!(logs[0]["action"], "version");
    assert_eq!(logs[0]["resp"], "9.9.9");
    assert_eq!(logs[0]["message"], "macprobe 9.9.9");
}

fn traced(responsible: Result<Proc, ProbeError>) -> Vec<serde_json::Value> {
    let c = cfg();
    let l = Lineage {
        responsible,
        procs: vec![
            Proc {
                pid: 9,
                path: Some("/a".into()),
            },
            Proc { pid: 1, path: None },
        ],
    };
    capture(LevelFilter::INFO, || {
        lineage(&ctx(), &c.messages, &c.lineage, &l)
    })
}

#[test]
fn lineage_found_is_info() {
    let c = cfg();
    let logs = traced(Ok(Proc {
        pid: 7,
        path: Some("/bin/x".into()),
    }));
    let e = &logs[0];
    assert_eq!(e["level"], "INFO");
    assert!(e.get("host").is_none());
    assert_eq!(e["feature"], c.lineage.id);
    assert_eq!(e["action"], "inspect");
    assert_eq!(e["resp"], "7 /bin/x");
    assert_eq!(e["chain"], "9 /a < 1");
    assert_eq!(
        e["message"],
        format!("{}{}7 /bin/x", c.lineage.name, c.messages.separator)
    );
}

#[test]
fn lineage_error_is_unknown() {
    let c = cfg();
    let logs = traced(Err(ProbeError::ResponsibleFailed));
    assert_eq!(logs[0]["resp"], "ResponsibleFailed");
    assert_eq!(
        logs[0]["message"],
        format!(
            "{}{}{}",
            c.lineage.name, c.messages.separator, c.messages.unknown
        )
    );
}

#[test]
fn priority_is_info() {
    let c = cfg();
    let logs = capture(LevelFilter::INFO, || {
        priority(&ctx(), &c.messages, &c.priority, "31", "9 /a=31")
    });
    let e = &logs[0];
    assert_eq!(e["level"], "INFO");
    assert_eq!(e["feature"], c.priority.id);
    assert_eq!(e["action"], "inspect");
    assert_eq!(e["resp"], "31");
    assert_eq!(e["tree"], "9 /a=31");
    assert_eq!(
        e["message"],
        format!("{}{}31", c.priority.name, c.messages.separator)
    );
}
