# TODOS (rolling)

This file tracks high-impact work items for this repo. Ordering is by priority (P0 highest).

## P0

- Fix TUI reliability after interrupts (`Esc` / "Conversation interrupted"): ensure the session always returns to a state where new user prompts submit and the UI can exit cleanly.
- Make `Ctrl+C` behavior predictable when idle (exit) and when running (interrupt), without getting stuck behind time-bounded shortcut windows.

## P1

- Investigate/fix Windows cross-build for `x86_64-pc-windows-msvc` (`ring` fails with `assert.h` missing under zigbuild); decide whether to ship GNU-only or support both.
- Add a reproducible integration test for `--auto-continue` when the agent omits the `AUTO_MODE_NEXT=...` marker (should still enqueue and submit the follow-up prompt unless an explicit `stop` is present).

## P2

- Decide whether `rg` should be bundled for Windows/Linux distributions (currently only `apply_patch` is bundled in the x64 zips).

## Done

- Hardened `--auto-continue` flow across `/new` + resume/fork, and made `AUTO_MODE_NEXT=stop` behave as a temporary pause even when `TurnStarted` isn’t observed.
- Improved error handling so `--auto-continue` can enqueue on turn-ending `EventMsg::Error` (only when a turn is actually running).
- Built, codesigned, and installed macOS `codex` (Homebrew prefix), and produced versioned Linux x64 + Windows x64 zip artifacts in `dist/`.
