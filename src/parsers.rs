use std::collections::HashMap;

pub fn read_file_str(path: &str) -> String {
    std::fs::read_to_string(path).expect("should have been able to read the file")
}

pub fn parse_colon_file(contents: &str, signal: bool) -> HashMap<String, String> {
    let mut info = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.splitn(2, ':');

        let key = match parts.next() {
            Some(key) => key.trim(),
            None => continue,
        };

        let value = match parts.next() {
            Some(value) => value.trim(),
            None => continue,
        };

        info.insert(key.to_string(), value.to_string());
        if signal {
            if key.to_string() == "cpu cores" {
                break;
            }
        }
    }

    info
}

pub fn parse_equals_file(contents: &str) -> HashMap<String, String> {
    let mut info = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.splitn(2, '=');

        let key = match parts.next() {
            Some(key) => key.trim(),
            None => continue,
        };

        let value = match parts.next() {
            Some(value) => value.trim().trim_matches('"'),
            None => continue,
        };

        info.insert(key.to_string(), value.to_string());
    }

    info
}

pub fn parse_number(value: &str) -> u64 {
    value
        .split_whitespace()
        .next()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}
