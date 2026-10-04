use std::env;

use crate::command::run_command;

pub fn session_type() -> String {
    env::var("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_lowercase()
}

pub fn detect_window_manager() -> Option<String> {
    match session_type().as_str() {
        "x11" => x11_window_manager(),
        "wayland" => detect_wayland_compositor(),
        _ => {
            // Fallback: if a Wayland socket exists, treat it as Wayland;
            // if only an X11 display exists, treat it as X11.
            if env::var("WAYLAND_DISPLAY").is_ok() {
                detect_wayland_compositor()
            } else if env::var("DISPLAY").is_ok() {
                x11_window_manager()
            } else {
                None
            }
        }
    }
}

fn x11_window_manager() -> Option<String> {
    let output = run_command("xprop -root _NET_WM_NAME")?;
    output.split('"').nth(1).map(|s| s.to_string())
}

fn detect_wayland_compositor() -> Option<String> {
    let env_hints = [
        ("HYPRLAND_INSTANCE_SIGNATURE", "Hyprland"),
        ("SWAYSOCK", "Sway"),
        ("WAYFIRE_SOCKET", "Wayfire"),
        ("RIVER", "River"),
        ("NIRI_SOCKET", "niri"),
    ];

    for (var, name) in &env_hints {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                return Some(name.to_string());
            }
        }
    }

    let commands = [
        ("hyprctl version", "Hyprland"),
        ("sway -v", "Sway"),
        ("niri --version", "niri"),
        ("river -version", "River"),
        ("cosmic-comp --version", "COSMIC"),
        ("gnome-shell --version", "GNOME Shell / Mutter"),
        ("mutter --version", "Mutter"),
    ];

    for (cmd, name) in &commands {
        if run_command(cmd).is_some() {
            return Some(name.to_string());
        }
    }

    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    if !desktop.is_empty() {
        return Some(desktop);
    }

    None
}
