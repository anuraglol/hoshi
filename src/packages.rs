use std::path::Path;
use std::process::Command;
use std::thread;

fn count_command_output(cmd: &str) -> Option<u64> {
    let mut parts = cmd.split_whitespace();
    let program = parts.next()?;

    let output = Command::new(program).args(parts).output().ok()?;

    if output.status.success() {
        Some(output.stdout.iter().filter(|&&b| b == b'\n').count() as u64)
    } else {
        None
    }
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

fn nix_packages() -> Option<u64> {
    let db = "/nix/var/nix/db/db.sqlite";
    if !Path::new(db).is_file() {
        return None;
    }

    let connection = sqlite::Connection::open_with_flags(
        // The nix store is immutable, so we need to inform sqlite about it
        "file:".to_owned() + db + "?immutable=1",
        sqlite::OpenFlags::new().with_read_only().with_uri(),
    );

    if let Ok(con) = connection {
        let statement = con.prepare("SELECT COUNT(path) FROM ValidPaths WHERE sigs IS NOT NULL");

        if let Ok(mut s) = statement {
            if s.next().is_ok() {
                return match s.read::<Option<i64>, _>(0) {
                    Ok(Some(count)) => Some(count as u64),
                    _ => None,
                };
            }
        }
    }

    None
}
