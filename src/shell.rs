use sysinfo::{Pid, System};

pub fn display_shell_info() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let pid = Pid::from_u32(std::process::id());
    if let Some(process) = sys.process(pid) {
        if let Some(ppid) = process.parent() {
            if let Some(shell_process) = sys.process(ppid) {
                println!(
                    "Executed from shell: {}",
                    shell_process.name().to_string_lossy()
                );
                if let Some(terminal_pid) = shell_process.parent() {
                    if let Some(terminal_process) = sys.process(terminal_pid) {
                        println!(
                            "Terminal Application: {}",
                            terminal_process.name().to_string_lossy()
                        );
                    }
                }
            }
        }
    }
}
