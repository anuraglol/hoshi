use std::thread::{self, ScopedJoinHandle};

use crate::types::BatInfo;

enum BatteryJob<'scope> {
    Wsl(ScopedJoinHandle<'scope, BatInfo>),
    Linux(
        ScopedJoinHandle<'scope, String>,
        ScopedJoinHandle<'scope, String>,
    ),
}

mod packages;
mod parsers;
mod renderer;
mod session;
mod shell;
mod types;
mod utils;

fn main() {
    let is_wsl = utils::is_wsl();

    let output = thread::scope(|s| {
        let battery_job = if is_wsl {
            BatteryJob::Wsl(s.spawn(|| utils::get_wsl_battery_info()))
        } else {
            BatteryJob::Linux(
                s.spawn(|| parsers::read_file_str("/sys/class/power_supply/BAT0/capacity")),
                s.spawn(|| parsers::read_file_str("/sys/class/power_supply/BAT0/status")),
            )
        };

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
        let disk_handle = s.spawn(|| {
            parsers::parse_disk_info().unwrap_or(types::DiskInfo {
                size: 0.0,
                used: 0.0,
                used_per: 0,
            })
        });

        types::Output {
            uname: uname_handle.join().unwrap(),
            hostname: hostname_handle.join().unwrap(),
            os_pretty_name: os_handle.join().unwrap(),
            uptime_seconds: uptime_handle.join().unwrap(),
            cpu_info: cpu_handle.join().unwrap(),
            mem_info: mem_handle.join().unwrap(),
            shell_info: shell_handle.join().unwrap(),
            packages_info: packages_handle.join().unwrap(),
            disk_info: disk_handle.join().unwrap(),

            bat_info: match battery_job {
                BatteryJob::Wsl(handle) => handle.join().unwrap(),
                BatteryJob::Linux(charge_handle, status_handle) => BatInfo {
                    current_charge: charge_handle.join().unwrap(),
                    battery_status: status_handle.join().unwrap(),
                },
            },
        }
    });

    renderer::display_output(&output);
}
