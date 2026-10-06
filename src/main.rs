use std::thread;

mod packages;
mod parsers;
mod renderer;
mod session;
mod shell;
mod types;

fn main() {
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
            parsers::parse_uptime_seconds(&contents)
        });
        let cpu_handle =
            s.spawn(|| parsers::parse_cpu_info(&parsers::read_file_str("/proc/cpuinfo")));
        let mem_handle =
            s.spawn(|| parsers::parse_mem_info(&parsers::read_file_str("/proc/meminfo")));

        let shell_handle = s.spawn(|| shell::shell_info());

        let packages_handle = s.spawn(|| packages::get_package_counts());

        types::Output {
            uname: uname_handle.join().unwrap(),
            hostname: hostname_handle.join().unwrap(),
            os_pretty_name: os_handle.join().unwrap(),
            uptime_seconds: uptime_handle.join().unwrap(),
            current_charge: current_charge_handle.join().unwrap(),
            battery_status: battery_status_handle.join().unwrap(),
            cpu_info: cpu_handle.join().unwrap(),
            mem_info: mem_handle.join().unwrap(),
            shell_info: shell_handle.join().unwrap(),
            packages_info: packages_handle.join().unwrap(),
        }
    });

    renderer::display_output(&output);
}
