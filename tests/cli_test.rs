use serde_json::Value;
use std::process::{Command, Output};

fn run(args: &[&str], level: Option<&str>) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_macprobe"));
    cmd.args(args).env_remove("MACPROBE_LOG");
    if let Some(l) = level {
        cmd.env("MACPROBE_LOG", l);
    }
    cmd.output().unwrap()
}

fn lines(out: &Output) -> Vec<Value> {
    let text = String::from_utf8(out.stdout.clone()).unwrap();
    text.lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn field<'a>(logs: &'a [Value], key: &str) -> Vec<&'a str> {
    logs.iter().map(|e| e[key].as_str().unwrap()).collect()
}

#[test]
fn default_outputs_info_json() {
    let out = run(&[], None);
    assert!([0, 1, 2].contains(&out.status.code().unwrap()));
    let logs = lines(&out);
    assert!(logs.iter().all(|e| e["level"] == "INFO"));
    assert_eq!(
        field(&logs, "feature"),
        [
            "responsible_process",
            "process_priority",
            "developer_tool",
            "full_disk_access",
            "accessibility",
            "debug_authorization"
        ]
    );
}

#[test]
fn debug_level_shows_details() {
    let logs = lines(&run(&[], Some("debug")));
    let debug: Vec<Value> = logs.into_iter().filter(|e| e["level"] == "DEBUG").collect();
    assert_eq!(
        field(&debug, "action"),
        [
            "responsible",
            "priority",
            "preflight",
            "open",
            "query",
            "exec"
        ]
    );
}

#[test]
fn args_exit_unknown_with_error() {
    let out = run(&["--json"], None);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(lines(&out)[0]["level"], "ERROR");
}

#[test]
fn version_flags_print_version() {
    for flag in ["--version", "-V"] {
        let out = run(&[flag], None);
        assert_eq!(out.status.code(), Some(0));
        let logs = lines(&out);
        assert_eq!(logs.len(), 1);
        assert_eq!(
            logs[0]["message"],
            format!("macprobe {}", env!("CARGO_PKG_VERSION"))
        );
    }
}
