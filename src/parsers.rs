use crate::types::{CpuInfo, MemInfo};

pub fn read_file_str(path: &str) -> String {
    std::fs::read_to_string(path).expect("should have been able to read the file")
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
    let mut cached = 0f64;
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
                "MemFree" => {
                    mem_free = value;
                    found |= 2;
                }
                "Cached" => {
                    cached = value;
                    found |= 4;
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
        cached,
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

pub fn parse_uptime_seconds(contents: &str) -> &str {
    contents.split_whitespace().next().unwrap_or("0")
}

pub fn parse_number(value: &str) -> u64 {
    value
        .split_whitespace()
        .next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}
