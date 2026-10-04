use std::process::Command;
use std::thread;

mod command;
mod display;
mod parsers;
mod renderer;
mod session;
mod shell;
mod types;

fn main() {
    let session_type = session::session_type();
    let window_manager = session::detect_window_manager();

    let output = thread::scope(|s| {
        let current_charge_handle =
            s.spawn(|| parsers::read_file_str("/sys/class/power_supply/BAT0/capacity"));
        let battery_status_handle =
            s.spawn(|| parsers::read_file_str("/sys/class/power_supply/BAT0/status"));
        let uname_handle = s.spawn(|| parsers::read_file_str("/proc/sys/kernel/osrelease"));
        let hostname_handle = s.spawn(|| parsers::read_file_str("/etc/hostname"));
        let os_handle =
            s.spawn(|| parsers::parse_os_pretty_name(&parsers::read_file_str("/etc/os-release")));
        let uptime_handle = s.spawn(|| {
            let contents = parsers::read_file_str("/proc/uptime");
            parsers::parse_uptime_seconds(&contents).to_string()
        });
        let cpu_handle =
            s.spawn(|| parsers::parse_cpu_info(&parsers::read_file_str("/proc/cpuinfo")));
        let mem_handle =
            s.spawn(|| parsers::parse_mem_info(&parsers::read_file_str("/proc/meminfo")));
        let flatpak_handle = s.spawn(|| {
            let output = Command::new("flatpak").arg("list").output();
            match output {
                Ok(output) if output.status.success() => {
                    String::from_utf8_lossy(&output.stdout).lines().count() as u64
                }
                _ => 0,
            }
        });
        let display_handle =
            s.spawn(|| display::get_displays(&session_type, window_manager.as_deref()));
        let shell_handle = s.spawn(|| shell::shell_info());

        types::Output {
            uname: uname_handle.join().unwrap(),
            hostname: hostname_handle.join().unwrap(),
            os_pretty_name: os_handle.join().unwrap(),
            uptime_seconds: uptime_handle.join().unwrap(),
            current_charge: current_charge_handle.join().unwrap(),
            battery_status: battery_status_handle.join().unwrap(),
            cpu_info: cpu_handle.join().unwrap(),
            mem_info: mem_handle.join().unwrap(),
            fp_count: flatpak_handle.join().unwrap(),
            displays: display_handle.join().unwrap(),
            shell_info: shell_handle.join().unwrap(),
        }
    });

    renderer::display_output(&output);
}
