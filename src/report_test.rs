use super::*;
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
    assert_eq!(e["user"], "tester");
    assert_eq!(e["host"], "term");
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
