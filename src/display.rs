use std::env;

use crate::command::run_command;
use crate::session::{detect_window_manager, session_type};

pub fn get_displays() -> Vec<String> {
    let on_wayland = env::var("WAYLAND_DISPLAY").is_ok() || session_type() == "wayland";
    let on_x11 = !on_wayland && (session_type() == "x11" || env::var("DISPLAY").is_ok());

    if on_x11 {
        return parse_xrandr_verbose(&run_command("xrandr --verbose").unwrap_or_default());
    }

    if on_wayland {
        let wm = detect_window_manager()
            .unwrap_or_default()
            .to_lowercase();

        if wm.contains("niri") {
            return parse_niri_outputs(&run_command("niri msg outputs").unwrap_or_default());
        }

        if wm.contains("hyprland") {
            return parse_hyprctl_monitors(&run_command("hyprctl monitors").unwrap_or_default());
        }

        // Generic wlroots-based compositor fallback.
        return parse_wlr_randr(&run_command("wlr-randr").unwrap_or_default());
    }

    Vec::new()
}

fn parse_xrandr_verbose(output: &str) -> Vec<String> {
    let mut displays = Vec::new();
    let mut lines = output.lines().peekable();

    while let Some(line) = lines.next() {
        if line.starts_with(' ') || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 || !parts[1].starts_with("connected") {
            continue;
        }

        let name = parts[0];
        let mut resolution = String::new();

        for part in &parts[2..] {
            if let Some(x_pos) = part.find('x') {
                let after_x = &part[x_pos + 1..];
                if after_x.find('+').is_some() {
                    if let Some(plus_pos) = part.find('+') {
                        resolution = part[..plus_pos].to_string();
                    }
                    break;
                }
            }
        }

        let mut scale_x = 1.0_f64;
        let mut scale_y = 1.0_f64;

        while let Some(next) = lines.peek() {
            if !next.starts_with(' ') {
                break;
            }

            if next.trim().starts_with("Transform:") {
                let _ = lines.next();
                if let Some(row1) = lines.next() {
                    scale_x = row1
                        .split_whitespace()
                        .next()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(1.0);
                }
                if let Some(row2) = lines.next() {
                    scale_y = row2
                        .split_whitespace()
                        .nth(1)
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(1.0);
                }
                break;
            }

            lines.next();
        }

        let scale = if (scale_x - scale_y).abs() < 0.001 {
            format!("{:.2}", scale_x)
        } else {
            format!("{:.2}x{:.2}", scale_x, scale_y)
        };

        displays.push(format!(
            "{}: {} (scale {})",
            name,
            if resolution.is_empty() {
                "unknown"
            } else {
                &resolution
            },
            scale
        ));
    }

    displays
}

fn parse_wlr_randr(output: &str) -> Vec<String> {
    let mut displays = Vec::new();
    let mut lines = output.lines().peekable();

    let mut current_name = String::new();
    let mut current_mode = String::new();
    let mut current_scale = "1.00".to_string();
    let mut enabled = false;

    fn finalize(name: &str, mode: &str, scale: &str, enabled: bool) -> Option<String> {
        if !enabled || name.is_empty() {
            return None;
        }

        Some(if mode.is_empty() {
            format!("{}: unknown resolution (scale {})", name, scale)
        } else {
            format!("{}: {} (scale {})", name, mode, scale)
        })
    }

    while let Some(line) = lines.next() {
        if line.starts_with(' ') {
            let trimmed = line.trim();

            if trimmed.starts_with("Enabled:") {
                enabled = trimmed.split_whitespace().nth(1) == Some("yes");
            } else if trimmed.starts_with("Modes:") {
                while let Some(mline) = lines.peek() {
                    if !mline.starts_with(' ') {
                        break;
                    }

                    let m = mline.trim();
                    if m.contains("current") {
                        if let Some(mode) = m.split_whitespace().next() {
                            current_mode = mode.to_string();
                        }
                        break;
                    }

                    lines.next();
                }
            } else if trimmed.starts_with("Scale:") {
                if let Some(scale) = trimmed.split_whitespace().nth(1) {
                    current_scale = scale.to_string();
                }
            }
        } else {
            if let Some(entry) = finalize(&current_name, &current_mode, &current_scale, enabled) {
                displays.push(entry);
            }

            current_name = line.split_whitespace().next().unwrap_or("").to_string();
            current_mode.clear();
            current_scale = "1.00".to_string();
            enabled = false;
        }
    }

    if let Some(entry) = finalize(&current_name, &current_mode, &current_scale, enabled) {
        displays.push(entry);
    }

    displays
}

fn parse_niri_outputs(output: &str) -> Vec<String> {
    let mut displays = Vec::new();

    let mut name = String::new();
    let mut mode = String::new();
    let mut scale = String::new();

    fn finalize(name: &str, mode: &str, scale: &str) -> Option<String> {
        if name.is_empty() {
            return None;
        }

        Some(format!(
            "{}: {} (scale {})",
            name,
            if mode.is_empty() { "unknown" } else { mode },
            if scale.is_empty() { "1" } else { scale }
        ))
    }

    for line in output.lines() {
        if line.starts_with("Output ") {
            if let Some(entry) = finalize(&name, &mode, &scale) {
                displays.push(entry);
            }

            name = line
                .split('(')
                .nth(1)
                .and_then(|s| s.strip_suffix(')'))
                .map(|s| s.trim())
                .unwrap_or("")
                .to_string();

            mode.clear();
            scale.clear();
        } else if line.trim().starts_with("Current mode:") {
            let after = line
                .trim()
                .strip_prefix("Current mode:")
                .unwrap_or("")
                .trim();
            mode = after
                .split(" @")
                .next()
                .and_then(|s| s.split_whitespace().next())
                .unwrap_or(after)
                .to_string();
        } else if line.trim().starts_with("Scale:") {
            scale = line
                .trim()
                .strip_prefix("Scale:")
                .unwrap_or("")
                .trim()
                .to_string();
        }
    }

    if let Some(entry) = finalize(&name, &mode, &scale) {
        displays.push(entry);
    }

    displays
}

fn parse_hyprctl_monitors(output: &str) -> Vec<String> {
    let mut displays = Vec::new();
    let mut lines = output.lines().peekable();

    while let Some(line) = lines.next() {
        if !line.starts_with("Monitor ") {
            continue;
        }

        let name = line.split_whitespace().nth(1).unwrap_or("").to_string();
        let mut mode = String::new();
        let mut scale = "1".to_string();

        while let Some(next) = lines.peek() {
            if next.starts_with("Monitor ") {
                break;
            }

            let trimmed = next.trim_start();
            if mode.is_empty() && trimmed.contains('@') && trimmed.contains(" at ") {
                mode = trimmed.split_whitespace().next().unwrap_or("").to_string();
            } else if trimmed.starts_with("scale:") {
                scale = trimmed
                    .strip_prefix("scale:")
                    .unwrap_or("")
                    .trim()
                    .to_string();
            }

            lines.next();
        }

        displays.push(format!(
            "{}: {} (scale {})",
            name,
            if mode.is_empty() { "unknown" } else { &mode },
            scale
        ));
    }

    displays
}
