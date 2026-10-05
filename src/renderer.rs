use crate::session;
use crate::types::Output;
use std::env;

const ASCII_ART: &str = r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠄⠒⠂⠀⠈⠀⠐⢒⡤⠄⠘⠂⠈⠉⠉⠠⠔⠋⠉⠀⠀⠀⠀⠀⠉⠐⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠎⠀⣀⣄⡀⠀⠀⠀⠶⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠒⠊⠣⡀⠀⠀⣣⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠂⠹⣟⠀⡨⠱⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⢨⡷⣤⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⠊⠀⢰⣉⠆⠢⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⢔⣉⡆⠀⠀⠣⣠⡶⡑⡼⢢⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠄⠢⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣷⡼⠁⠀⠡⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠊⠀⢠⠃⠀⠀⠀⠀⠀⠀⡇⠀⠀⡄⠀⠀⣄⠀⡾⡀⠀⠀⠀⡇⠀⠀⠀⠀⠘⡆⠀⠀⠀⢷⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢀⠊⠀⡔⢂⠇⠀⠀⢀⠀⠀⠀⢸⢸⠠⠞⠃⠀⠀⠛⡤⡇⡇⠀⠀⢠⡇⠀⠀⠀⠀⠀⢰⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠈⠄⠀⠇⠸⠀⠀⠀⢸⡀⠀⠀⢺⠈⡄⠀⠀⠀⠀⠀⢠⠁⢰⠀⠀⢸⠇⠀⠀⠀⠀⠀⠈⡆⠀⠀⠀⠘⣆⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠈⠂⠰⡇⠀⠀⠀⡈⢹⠀⠀⣼⡀⢱⠀⠀⠀⠀⠀⣼⣀⢸⠀⠀⡆⢸⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⢹⠢⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢀⠤⣒⣷⠀⠀⠀⡇⠀⢱⣴⣿⠋⠀⢳⠀⠀⠀⢀⠋⣙⣿⣶⣼⡀⠸⠀⠀⡸⠀⠀⠀⡇⠀⠀⠀⠀⠀⡆⠡⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣔⠔⠀⡊⢸⡀⡀⠀⣇⣴⢟⣿⣾⣿⣆⠀⢣⠀⠀⡎⣾⣟⣿⣼⣏⠙⢿⣦⡠⠃⠀⢀⣀⣔⡀⠀⠀⠀⠀⢱⠀⠡⠀
⠀⠀⠀⠀⠀⠀⢰⠂⠀⠀⢑⣸⠹⡧⡀⣿⠁⣸⠟⠛⠛⡻⠀⢠⡑⡜⠀⢟⠉⠉⠉⣻⡀⢺⠟⡇⢶⠭⠥⠤⣾⠀⠀⠀⠀⠀⠈⡄⠀⢡
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣸⠭⠭⣸⠚⠀⢊⣒⠒⠊⠁⠀⠚⠓⠛⠀⠀⠉⣭⢹⣁⠩⠔⠚⠓⠚⠛⠛⠣⢧⣇⠀⠀⠀⠀⠀⢱⠀⠀
⠀⠀⠀⠀⠀⢀⡠⠄⠂⠉⢁⠼⡒⠈⠑⠠⠤⣈⣉⣀⣀⣀⣀⣀⣀⣀⣀⣀⠄⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣆⠀⠀⠀⠀⠈⠆⠀
⠀⠀⠀⡠⠂⠁⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⡇⢠⠠⠄⠂⠤⠐⡘⢓⡞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠼⡆⠀⠀⠀⠀⠘⡀
⠀⡠⠚⠠⢄⡀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⡇⢂⢐⣈⣔⣆⠡⠐⡼⠐⠒⠂⠤⣀⠀⠀⢀⠠⠤⠀⠒⠈⣉⠀⠤⠐⢻⣀⠀⠀⠀⠀⠡
⢀⠯⡀⠀⠀⠈⠑⠢⡀⠀⠀⢀⠠⢇⢰⡀⠀⢗⡩⢣⠜⡠⢷⢈⡐⣇⣀⠀⠀⠀⠀⠉⠒⢄⡂⠀⠈⠉⠀⢀⡠⣄⡀⣸⠈⠢⣀⠀⠀⠘
⠘⠰⡄⣱⣢⣄⠀⠀⠈⣂⣔⠥⠒⠉⠺⡙⢠⢼⠰⡡⢊⠾⣉⢢⠄⠳⠠⡍⠁⠒⠠⠄⣀⠀⢀⠦⢤⣒⣊⠭⠝⠓⠉⠀⠀⠀⠈⠂⢤⠂
⠀⠀⠀⠁⠑⠐⠉⠂⠚⠉⠀⠀⠀⠀⠀⠀⢸⠌⡑⢆⡩⣼⠚⠤⣜⡠⠁⠜⡉⡖⠚⢒⠡⠉⠅⢂⠳⢀⣀⡲⠾⠂⠀⠀⠀⠀⠀⠀⠀⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡆⢎⠱⡨⡐⣽⣉⣐⡆⠀⠉⠉⠒⢻⠨⠤⢂⣉⡐⠂⡧⠤⠤⠤⠔⠂⠀⠀⠀⠀⠀⠀⢠⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡏⡜⠰⣁⠖⡠⢯⡱⠀⠀⠀⠀⠀⢸⠠⠄⠁⠀⠀⠀⠈⢐⣒⣒⠲⠇⠀⢀⣀⠤⢶⢤⠜⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠬⣑⠢⢌⡑⢢⡇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠈⠒⠒⠒⠒⠉⠁⡇⢎⠼⠧⣀⣀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢐⠨⡑⠤⠓⣌⠸⢄⡇⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡏⢆⠪⡑⠤⠀⠀
"#;

pub fn display_output(output: &Output) {
    let Output {
        hostname,
        uname,
        os_pretty_name,
        uptime_seconds,
        current_charge,
        battery_status,
        shell_info,
        cpu_info,
        mem_info,
        displays,
        packages_info,
    } = output;

    let session_type = session::session_type();
    let window_manager = session::detect_window_manager();

    let mut info = Vec::new();

    info.push(format!("hostname: {}", hostname.trim()));
    info.push(format!("kernel: {}", uname.trim()));
    info.push(format!("os: {}", os_pretty_name));
    info.push(format!("uptime: {}", uptime_seconds.trim()));
    info.push(format!("battery: {}%", current_charge.trim()));
    info.push(format!("bat status: {}", battery_status.trim()));
    info.push(String::new());

    if let Some((shell, terminal)) = shell_info {
        info.push(format!("shell: {}", shell));

        if let Some(term) = terminal {
            info.push(format!("terminal: {}", term));
        }

        info.push(String::new());
    }

    info.push(format!("model: {}", cpu_info.model_name));
    info.push(format!("cores: {}", cpu_info.cpu_cores));
    info.push(format!(
        "memory: {:.2}GB / {:.2}GB, cached: {:.2}GB",
        mem_info.mem_free, mem_info.mem_total, mem_info.cached
    ));
    info.push(format!(
        "swap: {:.2}GB / {:.2}GB",
        mem_info.swap_free, mem_info.swap_total
    ));
    info.push(String::new());

    info.push(format!("session type: {}", session_type));
    info.push(format!(
        "desktop: {}",
        env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
    ));
    info.push(format!(
        "desktop session: {}",
        env::var("DESKTOP_SESSION").unwrap_or_default()
    ));
    info.push(format!(
        "DISPLAY: {}",
        env::var("DISPLAY").unwrap_or_default()
    ));
    info.push(format!(
        "WAYLAND_DISPLAY: {}",
        env::var("WAYLAND_DISPLAY").unwrap_or_default()
    ));
    info.push(format!(
        "wm/compositor: {}",
        window_manager.as_deref().unwrap_or("unknown")
    ));
    if displays.is_empty() {
        info.push("no display information available".to_string());
    } else {
        for display in displays {
            info.push(display.to_string());
        }
    }

    info.push(String::new());
    for (manager, count) in packages_info {
        info.push(format!("{} packages: {}", manager, count));
    }

    let art_lines: Vec<&str> = ASCII_ART.lines().collect();

    let art_width = art_lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let total_lines = art_lines.len().max(info.len());

    println!();
    for i in 0..total_lines {
        let art = art_lines.get(i).copied().unwrap_or("");
        let stat = info.get(i).map(String::as_str).unwrap_or("");

        println!("{:<width$}        {}", art, stat, width = art_width);
    }
    println!();
}
