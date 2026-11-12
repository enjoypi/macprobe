use super::*;

fn run(command: &str, args: &[&str]) -> Result<String, ProbeError> {
    let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
    stdout(command, &args)
}

#[test]
fn returns_stdout() {
    assert_eq!(run("/bin/echo", &["hi"]), Ok("hi\n".into()));
}

#[test]
fn spawn_failure() {
    assert_eq!(run("/nonexistent", &[]), Err(ProbeError::SpawnFailed));
}

#[test]
fn nonzero_exit_still_returns_stdout() {
    assert_eq!(run("/usr/bin/false", &[]), Ok(String::new()));
}
