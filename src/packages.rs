use std::io::BufRead;
use std::process::Command;
use std::{env, thread};

fn count_command_output(cmd: &str) -> Option<u64> {
    let mut parts = cmd.split_whitespace();
    let program = parts.next()?;

    let output = Command::new(program).args(parts).output().ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Some(stdout.lines().count() as u64)
    } else {
        None
    }
}

fn nix_store_references(paths: &[&str]) -> Option<u64> {
    let output = Command::new("nix-store")
        .args(["-q", "--references"])
        .args(paths)
        .output()
        .ok()?;

    if output.status.success() {
        let count = output.stdout.lines().count() as u64;

        Some(count.saturating_sub(1))
    } else {
        None
    }
}

fn nix_packages() -> Option<u64> {
    let mut paths = vec!["/run/current-system/sw"];
    let mut user_profile = None;

    if let Ok(home) = env::var("HOME") {
        let profile = format!("{home}/.nix-profile");

        if std::path::Path::new(&profile).exists() {
            user_profile = Some(profile);
        }
    }

    if let Some(ref profile) = user_profile {
        paths.push(profile);
    }

    let total = nix_store_references(&paths).unwrap_or(0);

    if total > 0 { Some(total) } else { None }
}

pub fn get_package_counts() -> Vec<(&'static str, u64)> {
    let command_managers = [
        ("flatpak", "flatpak list"),
        ("pacman", "pacman -Q"),
        ("apt", "dpkg -l"),
    ];

    thread::scope(|scope| {
        let mut handles = Vec::new();

        for &(name, cmd) in &command_managers {
            handles.push(scope.spawn(move || count_command_output(cmd).map(|count| (name, count))));
        }

        handles.push(scope.spawn(|| nix_packages().map(|count| ("nix", count))));

        handles
            .into_iter()
            .filter_map(|handle| handle.join().ok().flatten())
            .collect()
    })
}
