use super::*;

#[test]
fn embedded_has_expected_values() {
    let c = embedded();
    assert_eq!(c.tcc.service, "kTCCServiceDeveloperTool");
    assert_eq!(c.tcc.symbol, "TCCAccessPreflight");
    assert_eq!(c.tcc.cf_encoding, 0x0800_0100);
    assert_eq!(c.tcc.granted, vec![0]);
    assert_eq!(c.tcc.denied, vec![1, 2]);
    assert_eq!(
        c.fda.path,
        "/Library/Application Support/com.apple.TCC/TCC.db"
    );
    assert_eq!(c.fda.denied_errno, vec![1]);
    assert_eq!(c.ax.id, "accessibility");
    assert_eq!(c.lineage.id, "responsible_process");
    assert_eq!(
        c.lineage.symbol,
        "responsibility_get_pid_responsible_for_pid"
    );
    assert_eq!((c.lineage.root_pid, c.lineage.shortbsdinfo_flavor), (1, 13));
    assert_eq!(c.priority.id, "process_priority");
    assert_eq!(
        (c.priority.listthreads_flavor, c.priority.threadinfo_flavor),
        (6, 5)
    );
    assert_eq!(c.devtools.command, "/usr/sbin/DevToolsSecurity");
    assert_eq!(c.devtools.args, vec!["-status"]);
    assert_eq!((c.exit.granted, c.exit.denied, c.exit.unknown), (0, 1, 2));
    assert_eq!(c.exit.ok, 0);
    assert_eq!(c.cli.version_flags, vec!["--version", "-V"]);
    assert_eq!(c.log.level_env, "MACPROBE_LOG");
    assert_eq!(c.log.default_level, "info");
}

#[test]
fn parse_rejects_invalid_yaml() {
    assert!(parse("tcc: [").is_err());
}
