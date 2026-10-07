use std::fs;
use std::process::Command;

use crate::types::BatInfo;

pub fn is_wsl() -> bool {
    fs::read_to_string("/proc/version")
        .map(|v| {
            let v = v.to_ascii_lowercase();
            v.contains("microsoft") || v.contains("wsl")
        })
        .unwrap_or(false)
}

fn map_battery_status(status: u16) -> &'static str {
    match status {
        1 => "Discharging",
        2 => "Not charging",
        3 => "Full",
        4 => "Low",
        5 => "Critical",
        6 | 7 | 8 | 9 => "Charging",
        11 => "Partial",
        _ => "Unknown",
    }
}

pub fn get_wsl_battery_info() -> BatInfo {
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-Command",
            "Get-WmiObject -Class Win32_Battery | ForEach-Object { \"$($_.EstimatedChargeRemaining),$($_.BatteryStatus)\" }",
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);

            if let Some(line) = text.lines().find(|line| !line.trim().is_empty()) {
                let parts: Vec<&str> = line.split(',').collect();

                if parts.len() >= 2 {
                    let charge = parts[0].trim().to_string();
                    let status = parts[1].trim().parse::<u16>().unwrap_or(0);

                    return BatInfo {
                        current_charge: charge,
                        battery_status: map_battery_status(status).to_string(),
                    };
                }

                if parts.len() == 1 {
                    return BatInfo {
                        current_charge: parts[0].trim().to_string(),
                        battery_status: String::new(),
                    };
                }
            }
        }
        _ => {}
    }

    BatInfo {
        current_charge: "0".to_string(),
        battery_status: String::new(),
    }
}
