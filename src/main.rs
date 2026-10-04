use std::env;
use std::process::Command;
use std::thread;

mod command;
mod display;
mod parsers;
mod session;
mod shell;
mod types;

fn main() {
    // Fast environment-only lookups first; these are needed to pick the right
    // display command, so do them before the parallel block.
    let session_type = session::session_type();
    let window_manager = session::detect_window_manager();

    // Run all independent I/O-bound work in parallel.
    let (
        uname,
        hostname,
        os_pretty_name,
        uptime_seconds,
        cpu_info,
        mem_info,
        fp_count,
        displays,
        shell_info,
    ) = thread::scope(|s| {
        let uname_handle = s.spawn(|| parsers::read_file_str("/proc/sys/kernel/osrelease"));
        let hostname_handle = s.spawn(|| parsers::read_file_str("/etc/hostname"));
        let os_handle = s.spawn(|| {
            parsers::parse_os_pretty_name(&parsers::read_file_str("/etc/os-release"))
        });
        let uptime_handle = s.spawn(|| {
            let contents = parsers::read_file_str("/proc/uptime");
            parsers::parse_uptime_seconds(&contents).to_string()
        });
        let cpu_handle = s.spawn(|| {
            parsers::parse_cpu_info(&parsers::read_file_str("/proc/cpuinfo"))
        });
        let mem_handle = s.spawn(|| {
            parsers::parse_mem_info(&parsers::read_file_str("/proc/meminfo"))
        });
        let flatpak_handle = s.spawn(|| {
            let output = Command::new("flatpak").arg("list").output();
            match output {
                Ok(output) if output.status.success() => {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .count()
                        .saturating_sub(1) as u64
                }
                _ => 0,
            }
        });
        let display_handle = s.spawn(|| {
            display::get_displays(&session_type, window_manager.as_deref())
        });
        let shell_handle = s.spawn(|| shell::shell_info());

        (
            uname_handle.join().unwrap(),
            hostname_handle.join().unwrap(),
            os_handle.join().unwrap(),
            uptime_handle.join().unwrap(),
            cpu_handle.join().unwrap(),
            mem_handle.join().unwrap(),
            flatpak_handle.join().unwrap(),
            display_handle.join().unwrap(),
            shell_handle.join().unwrap(),
        )
    });

    println!("hostname: {}", hostname.trim());
    println!("kernel: {}", uname.trim());
    println!("os: {}", os_pretty_name);
    println!("uptime: {} seconds\n", uptime_seconds.trim());

    if let Some((shell, terminal)) = shell_info {
        println!("Executed from shell: {}", shell);
        if let Some(term) = terminal {
            println!("Terminal Application: {}", term);
        }
    }
    println!();

    println!("-----CPU-----");
    println!("model: {}", cpu_info.model_name);
    println!("cores: {}\n", cpu_info.cpu_cores);

    println!("-----Memory-----");
    println!(
        "memory: free/total, cached: {}/{}, {}",
        mem_info.mem_free_kb, mem_info.mem_total_kb, mem_info.cached_kb
    );
    println!(
        "swap: total, free: {}, {} kB",
        mem_info.swap_total_kb, mem_info.swap_free_kb
    );

    println!("\n-----Session / Window Manager-----");
    println!("session type: {}", session_type);
    println!(
        "desktop: {}",
        env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
    );
    println!(
        "desktop session: {}",
        env::var("DESKTOP_SESSION").unwrap_or_default()
    );
    println!("DISPLAY: {}", env::var("DISPLAY").unwrap_or_default());
    println!(
        "WAYLAND_DISPLAY: {}",
        env::var("WAYLAND_DISPLAY").unwrap_or_default()
    );
    println!(
        "wm/compositor: {}",
        window_manager.as_deref().unwrap_or("unknown")
    );

    println!("\n-----Display-----");
    if displays.is_empty() {
        println!("no display information available");
    } else {
        for display in displays {
            println!("{}", display);
        }
    }

    println!("\nflatpak packages({})", fp_count);
}
