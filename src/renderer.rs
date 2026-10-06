use crate::session;
use crate::types::Output;
use std::env;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const BLUE: &str = "\x1b[34m";

fn paint(text: &str, color: &str, no_color: bool) -> String {
    if no_color {
        text.to_string()
    } else {
        format!("{}{}{}", color, text, RESET)
    }
}

fn label(text: &str, no_color: bool) -> String {
    paint(text, &format!("{BOLD}{CYAN}"), no_color)
}

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
    let no_color = env::var("NO_COLOR").is_ok_and(|v| !v.is_empty());
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
        packages_info,
    } = output;

    let session_type = session::session_type();

    let mut info = Vec::new();

    info.push(format!(
        "{}: {}",
        label("hostname", no_color),
        hostname.trim()
    ));
    info.push(format!("{}: {}", label("kernel", no_color), uname.trim()));
    info.push(format!("{}: {}", label("os", no_color), os_pretty_name));
    info.push(format!(
        "{}: {}",
        label("uptime", no_color),
        uptime_seconds.trim()
    ));

    info.push(format!(
        "{}: {}% ({})",
        label("battery", no_color),
        current_charge.trim(),
        battery_status.trim()
    ));
    info.push(String::new());

    if let Some((shell, terminal)) = shell_info {
        info.push(format!("{}: {}", label("shell", no_color), shell));

        if let Some(term) = terminal {
            info.push(format!("{}: {}", label("terminal", no_color), term));
        }

        info.push(String::new());
    }

    info.push(format!(
        "{}: {}",
        label("model", no_color),
        cpu_info.model_name
    ));
    info.push(format!(
        "{}: {}",
        label("cores", no_color),
        cpu_info.cpu_cores
    ));
    info.push(format!(
        "{}: {:.2}GB / {:.2}GB, {}: {:.2}GB",
        label("memory", no_color),
        mem_info.mem_free,
        mem_info.mem_total,
        label("cached", no_color),
        mem_info.cached
    ));
    info.push(format!(
        "{}: {:.2}GB / {:.2}GB",
        label("swap", no_color),
        mem_info.swap_free,
        mem_info.swap_total
    ));
    info.push(String::new());

    info.push(format!(
        "{}: {}",
        label("session type", no_color),
        session_type
    ));
    info.push(format!(
        "{}: {}",
        label("desktop", no_color),
        env::var("XDG_CURRENT_DESKTOP").unwrap_or_default()
    ));

    if !packages_info.is_empty() {
        let package_str = packages_info
            .iter()
            .map(|(manager, count)| format!("{}({})", count, paint(manager, BLUE, no_color)))
            .collect::<Vec<_>>()
            .join(", ");
        info.push(format!("{}: {}", label("packages", no_color), package_str));
    }

    let art_lines: Vec<&str> = ASCII_ART
        .lines()
        .skip_while(|line| line.is_empty())
        .collect();

    let art_width = art_lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let (total_lines, offset) = if art_lines.len() >= info.len() {
        (art_lines.len(), (art_lines.len() - info.len()) / 2)
    } else {
        (info.len(), 0)
    };

    println!();
    for i in 0..total_lines {
        let art = art_lines.get(i).copied().unwrap_or("");
        let stat = if i >= offset && i - offset < info.len() {
            info[i - offset].as_str()
        } else {
            ""
        };

        println!("{:<width$}        {}", art, stat, width = art_width);
    }
    println!();
}
