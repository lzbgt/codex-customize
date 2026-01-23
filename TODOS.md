# TODOS (rolling)

This file tracks high-impact work items for this repo. Ordering is by priority (P0 highest).

## P0

- Rebase/merge this fork onto the latest upstream Codex CLI, then re-apply/augment the auto-continue feature + verbose defaults on top of upstream (rolling style; compatibility not required).

## P1

- Investigate/fix Windows cross-build for `x86_64-pc-windows-msvc` (`ring` fails with `assert.h` missing under zigbuild); decide whether to ship GNU-only or support both.

## P2

- Decide whether `rg` should be bundled for Windows/Linux distributions (currently only `apply_patch` is bundled in the x64 zips).

## Done

- Fixed TUI reliability after interrupts (`Esc` / "Conversation interrupted"): queued prompts submitted even if entered before SessionConfigured; interrupt clears stuck MCP startup running state.
- Made `Ctrl+C` behavior predictable when idle (exit) and when running (interrupt), without being blocked by MCP startup “running” state.
- Added a reproducible integration test for `--auto-continue` when the agent omits the `AUTO_MODE_NEXT=...` marker (should still enqueue and submit the follow-up prompt unless an explicit `stop` is present).
- Hardened `--auto-continue` flow across `/new` + resume/fork, and made `AUTO_MODE_NEXT=stop` behave as a temporary pause even when `TurnStarted` isn’t observed.
- Improved error handling so `--auto-continue` can enqueue on turn-ending `EventMsg::Error` (only when a turn is actually running).
- Made tool output fully visible by default (new `tui.show_full_tool_output`, no transcript ellipses for exec/MCP/patch failure output).
- Built, codesigned, and installed macOS `codex` (Homebrew prefix), and produced versioned Linux x64 + Windows x64 zip artifacts in `dist/`.
