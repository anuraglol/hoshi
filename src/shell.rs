use std::fs;

fn ppid_of(pid: u32) -> Option<u32> {
    let status = fs::read_to_string(format!("/proc/{}/status", pid)).ok()?;
    status
        .lines()
        .find(|line| line.starts_with("PPid:"))?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}

fn comm_of(pid: u32) -> Option<String> {
    fs::read_to_string(format!("/proc/{}/comm", pid))
        .ok()
        .map(|s| s.trim().to_string())
}

pub fn shell_info() -> Option<(String, Option<String>)> {
    let self_pid = std::process::id();
    let shell_pid = ppid_of(self_pid)?;
    let shell_name = comm_of(shell_pid)?;
    let terminal_pid = ppid_of(shell_pid)?;
    let terminal_name = comm_of(terminal_pid);

    Some((shell_name, terminal_name))
}
