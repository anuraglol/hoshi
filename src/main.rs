use std::env;
use std::process::Command;

mod command;
mod display;
mod parsers;
mod session;
mod shell;
mod types;

use types::{CpuInfo, MemInfo};

use crate::shell::display_shell_info;

fn main() {
    let uname = parsers::read_file_str("/proc/sys/kernel/osrelease");
    let cpuinfo = parsers::read_file_str("/proc/cpuinfo");
    let meminfo = parsers::read_file_str("/proc/meminfo");
    let hostname = parsers::read_file_str("/etc/hostname");
    let osinfo = parsers::read_file_str("/etc/os-release");
    let uptime = parsers::read_file_str("/proc/uptime");

    let cpu = parsers::parse_colon_file(&cpuinfo, true);
    let mem = parsers::parse_colon_file(&meminfo, false);
    let os = parsers::parse_equals_file(&osinfo);

    let cpu_info = CpuInfo {
        model_name: cpu.get("model name").cloned().unwrap_or_default(),

        cpu_cores: cpu
            .get("cpu cores")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
    };

    let mem_info = MemInfo {
        mem_total_kb: mem
            .get("MemTotal")
            .map(|v| parsers::parse_number(v))
            .unwrap_or(0),
        mem_free_kb: mem
            .get("MemFree")
            .map(|v| parsers::parse_number(v))
            .unwrap_or(0),
        cached_kb: mem
            .get("Cached")
            .map(|v| parsers::parse_number(v))
            .unwrap_or(0),
        swap_total_kb: mem
            .get("SwapTotal")
            .map(|v| parsers::parse_number(v))
            .unwrap_or(0),
        swap_free_kb: mem
            .get("SwapFree")
            .map(|v| parsers::parse_number(v))
            .unwrap_or(0),
    };

    let mut fp_count: u64 = 0;
    let fp_output = Command::new("flatpak")
        .arg("list")
        .output()
        .expect("failed to run");

    if fp_output.status.success() {
        let stdout = String::from_utf8_lossy(&fp_output.stdout);
        fp_count = stdout.lines().count().saturating_sub(1) as u64;
    }

    println!("hostname: {}", hostname.trim());
    println!("kernel: {}", uname.trim());
    println!("os: {}", os.get("PRETTY_NAME").cloned().unwrap_or_default());
    println!(
        "uptime: {} seconds\n",
        uptime.split(" ").next().unwrap_or(" ").trim()
    );
    display_shell_info();
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
    println!("session type: {}", session::session_type());
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
        session::detect_window_manager().unwrap_or_else(|| "unknown".to_string())
    );

    println!("\n-----Display-----");
    let displays = display::get_displays();
    if displays.is_empty() {
        println!("no display information available");
    } else {
        for display in displays {
            println!("{}", display);
        }
    }

    println!("\nflatpak packages({})", fp_count);
}
