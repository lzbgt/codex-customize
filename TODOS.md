# TODOS (rolling)

This file tracks high-impact work items for this repo. Ordering is by priority (P0 highest).

## P0

- Fix TUI reliability after interrupts (`Esc` / "Conversation interrupted"): ensure the session always returns to a state where new user prompts submit and the UI can exit cleanly.
- Make `Ctrl+C` behavior predictable when idle (exit) and when running (interrupt), without getting stuck behind time-bounded shortcut windows.
- Harden `--auto-continue` flow: enqueue exactly once per turn end, never mid-turn, and never get permanently disabled by a transient `AUTO_MODE_NEXT=stop` or user interrupt.

## P1

- Build/release artifacts: macOS codesigned `codex` install + Linux x64 + Windows x64 binaries.
- Add safe cleanup tooling for build/run artifacts (age-based cleanup with a dry-run mode).

## P2

- Evaluate whether `rg` / `apply_patch` helper tooling should be bundled or documented for Windows/Linux distributions.

