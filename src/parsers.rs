use std::process::Command;

use crate::types::{CpuInfo, DiskInfo, MemInfo};

fn run_command(cmd: &str) -> Option<String> {
    let mut parts = cmd.split_whitespace();
    let program = parts.next()?;
    let output = Command::new(program).args(parts).output().ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        None
    }
}

pub fn read_file_str(path: &str) -> String {
    std::fs::read_to_string(path).expect("should have been able to read the file")
}

pub fn parse_disk_info() -> Option<DiskInfo> {
    let output = run_command("df /")?;

    let line = output.lines().nth(1)?;
    let mut parts = line.split_whitespace();

    let _filesystem = parts.next()?;
    let size = parts.next()?.parse::<u64>().ok()? as f64 / 1024.0 / 1024.0;
    let used = parts.next()?.parse::<u64>().ok()? as f64 / 1024.0 / 1024.0;
    let _available = parts.next()?;
    let used_per = parts.next()?;
    parts.next()?;

    Some(DiskInfo {
        size,
        used,
        used_per: used_per[..used_per.len() - 1].parse().ok()?,
    })
}

pub fn parse_cpu_info(contents: &str) -> CpuInfo {
    let mut model_name = String::new();
    let mut cpu_cores = 0u32;

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            if model_name.is_empty() && key == "model name" {
                model_name = value.to_string();
            } else if cpu_cores == 0 && key == "cpu cores" {
                if let Ok(n) = value.split_whitespace().next().unwrap_or(value).parse() {
                    cpu_cores = n;
                }
            }

            if !model_name.is_empty() && cpu_cores != 0 {
                break;
            }
        }
    }

    CpuInfo {
        model_name,
        cpu_cores,
    }
}

pub fn parse_mem_info(contents: &str) -> MemInfo {
    let mut mem_total = 0f64;
    let mut mem_free = 0f64;
    let mut swap_total = 0f64;
    let mut swap_free = 0f64;
    let mut found = 0u8;

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let value = parse_number(value.trim()) as f64 / 1024.0 / 1024.0;

            match key.trim() {
                "MemTotal" => {
                    mem_total = value;
                    found |= 1;
                }
                "MemAvailable" => {
                    mem_free = value;
                    found |= 2;
                }
                "SwapTotal" => {
                    swap_total = value;
                    found |= 8;
                }
                "SwapFree" => {
                    swap_free = value;
                    found |= 16;
                }
                _ => {}
            }

            if found == 31 {
                break;
            }
        }
    }

    MemInfo {
        mem_total,
        mem_free,
        swap_total,
        swap_free,
    }
}

pub fn parse_os_pretty_name(contents: &str) -> String {
    for line in contents.lines() {
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "PRETTY_NAME" {
                return value.trim().trim_matches('"').to_string();
            }
        }
    }
    String::new()
}

pub fn parse_uptime_seconds(contents: &str) -> String {
    let total_seconds = contents
        .split_whitespace()
        .next()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0) as u64;

    let mut seconds = total_seconds;

    let years = seconds / 31_536_000;
    seconds %= 31_536_000;

    let months = seconds / 2_592_000;
    seconds %= 2_592_000;

    let days = seconds / 86_400;
    seconds %= 86_400;

    let hours = seconds / 3_600;
    seconds %= 3_600;

    let minutes = seconds / 60;

    let mut parts = Vec::new();

    if years > 0 {
        parts.push(format!("{} years", years));
    }
    if months > 0 {
        parts.push(format!("{} months", months));
    }
    if days > 0 {
        parts.push(format!("{} days", days));
    }
    if hours > 0 {
        parts.push(format!("{} hours", hours));
    }
    if minutes > 0 {
        parts.push(format!("{} mins", minutes));
    }

    parts.join(", ")
}

pub fn parse_number(value: &str) -> u64 {
    value
        .split_whitespace()
        .next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}
