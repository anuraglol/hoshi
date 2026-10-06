use std::env;

pub fn session_type() -> String {
    env::var("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_lowercase()
}
