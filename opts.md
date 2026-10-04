1.  Parallelize independent reads/commands
    Your file reads and shell commands don't depend on each other, but they run sequentially. Spawn threads or use rayon/tokio to run these
    concurrently:

- /proc/sys/kernel/osrelease, /proc/cpuinfo, /proc/meminfo, /etc/hostname, /etc/os-release, /proc/uptime
- flatpak list | wc -l
- display detection commands

2.  Cache detect_window_manager()
    It's called twice: once inside display::get_displays() and once in main. Compute it once and pass it around.

3.  Stop parsing files early
    /proc/cpuinfo and /proc/meminfo are huge on big systems. You only need model name, cpu cores, and a few mem keys. Scan lines and return as soon as
    you find what you need instead of building a full HashMap.

4.  Reduce allocations

- Avoid .to_string() everywhere; parse into &str slices where possible.
- In parse_xrandr_verbose, line.split_whitespace().collect() builds a Vec you mostly don't need—iterate directly.
- Return Vec<&str> or a small struct instead of formatted Strings from parsers, then format only at print time.

5.  Avoid shell indirection for flatpak
    Command::new("sh").arg("-c").arg("flatpak list | wc -l") spawns a shell. Run flatpak list directly and count its output lines in Rust.

6.  Lazily detect the compositor
    detect_wayland_compositor() runs a chain of run_command calls. Most of the time env vars are enough—move the command checks behind a "none of the
    env hints matched" branch (you already do env first, good). But also bail out as soon as one succeeds instead of continuing.

7.  Use a small struct instead of HashMap<String, String>
    For known keys like MemTotal, define struct MemInfo upfront and fill it while parsing, skipping the hashmap entirely.

8.  Avoid repeated session_type() and env::var calls
    get_displays() calls session_type() and env::var multiple times. Read them once at the top.

9.  Compile-time regex (if you add regex)
    Not needed here, but if parsing gets more complex, use lazy_static/once_cell regexes instead of recompiling per call.

10. Profile before over-optimizing
    Run cargo build --release and time it with hyperfine or time. Most of the runtime is probably waiting on external commands, not Rust code—so
    parallelism (suggestion 1) will give the biggest real-world speedup.
