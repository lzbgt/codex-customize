# TUI hang debugging (macOS)

This doc captures the fast path for collecting actionable data when the TUI appears to hang.

## Build a debug binary

```bash
cd codex-rs
cargo build -p codex-cli
```

Then run the debug binary in another terminal:

```bash
./target/debug/codex
```

## Capture a sample

1. Find the PID of the stuck process (for example with `ps` or Activity Monitor).
2. Run the helper script:

```bash
scripts/capture_tui_sample.sh <pid> 10
```

The sample output is written to `build/logs/sample_codex_<pid>_<timestamp>.txt` by default.

## Signal-triggered diagnostics (Unix)

Send `SIGUSR1` to emit a diagnostic snapshot (event broker stats + backtrace) into `codex-tui.log`:

```bash
kill -USR1 <pid>
```

This captures the stack of the TUI thread that receives the signal; it is not a full multi-thread dump.

## What to look for

- Watchdog logs in `codex-tui.log` for:
  - `polling_active_ms` (how long the crossterm poller was active)
  - `pause_calls` / `resume_calls`
  - `paused_ms` (time spent paused)
  - `active_buffer_len`, `buffered_events_total`, and `buffer_contended`
- In the sample output, locate the active thread stack and identify the blocking wait (e.g. a mutex lock, channel receive, or runtime park).

## Optional: lldb attach

If you need a live backtrace instead of a sample:

```bash
lldb -p <pid>
thread backtrace all
```

Save the backtrace output alongside the sample log for correlation.
