use crate::error::ProbeError;
use std::process::Command;

pub fn stdout(command: &str, args: &[String]) -> Result<String, ProbeError> {
    let out = Command::new(command)
        .args(args)
        .output()
        .map_err(|_| ProbeError::SpawnFailed)?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[cfg(test)]
#[path = "cmd_test.rs"]
mod tests;
