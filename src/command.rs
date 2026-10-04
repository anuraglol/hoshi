use std::process::Command;

pub fn run_command(cmd: &str) -> Option<String> {
    let mut parts = cmd.split_whitespace();
    let program = parts.next()?;
    let output = Command::new(program).args(parts).output().ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}
