Profiled 2025-10-05: total ~120ms, entirely bounded by the packages thread.

| piece                                    | time         |
|------------------------------------------|--------------|
| nix-store -q --references /run/current-system/sw | ~94ms |
| nix-store -q --references ~/.nix-profile (warm)  | ~30-40ms |
| flatpak list                             | ~29ms        |
| everything else (threads + WM detect)    | hidden, ~10ms |

1.  Parallelize inside get_package_counts()
    Managers run sequentially in ONE thread (flatpak -> pacman -> apt -> nix x2), so main's
    thread-per-module parallelism is wasted: this thread is the critical path. Spawn one scoped
    thread per manager so wall time = max(...) not sum(...). (~120ms -> ~95ms)

2.  Merge the two nix-store calls into one invocation
    `nix-store -q --references pathA pathB` accepts multiple paths. Measured: 63ms vs ~125ms for
    two calls (one sqlite DB open instead of two). Dedupe lines in Rust to keep the union count.
    Combined with #1: ~65ms total.

3.  Drop the `flatpak list` spawn, count dirs instead
    read_dir("/var/lib/flatpak/app") + .../runtime + ~/.local/share/flatpak/{app,runtime} is
    sub-millisecond vs 29ms for the subprocess.

4.  Count newlines on raw bytes
    `String::from_utf8_lossy(&output.stdout).lines().count()` allocates + walks the buffer as
    chars. Use `output.stdout.iter().filter(|&&b| b == b'\n').count()` — alloc-free. Matters for
    dpkg -l-sized outputs, free everywhere.

5.  detect_window_manager() still runs twice
    Once serially BEFORE thread::scope in main, once AFTER the joins in renderer::display_output
    (serial tail latency, directly additive). Compute once inside the scope, put it in Output,
    renderer consumes it. Cheap here (NIRI_SOCKET env fast-path) but on X11 it's two xprop spawns.

6.  Consolidate the ~8 trivial file-read threads in main into one
    Reading /etc/hostname etc. takes ~20us each; thread spawn+join costs more than the read.
    Only becomes visible after #1-#3 land.

Expected after #1-#3: ~65ms, floored by the single nix-store call. Going lower means bypassing
nix-store's sqlite open entirely (rusqlite dep or approximate counting) — not worth it.
