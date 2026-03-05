# TODOS (rolling)

This file tracks high-impact work items for this repo. Ordering is by priority (P0 highest).

## P0

- Investigate remaining TUI hangs in long-running sessions:
  - Capture symbolicated stacks from a debug build (`lldb` or `sample` on the debug binary).
  - Identify the blocking await/condvar and add watchdog metrics/logging around event queues.
  - Add a regression test or targeted fuzz case once the root cause is confirmed.
- Verify `--auto-continue` behavior in real sessions on the latest upstream base:
  - No-marker turn-end still triggers the follow-up prompt.
  - `AUTO_MODE_NEXT=stop` pauses auto-continue for that boundary only (next manual turn resumes).
  - No duplicate follow-up prompts during long turns / partial event streams.

## P1

- Investigate/fix Windows cross-build for `x86_64-pc-windows-msvc` (`ring` fails with `assert.h` missing under zigbuild); decide whether to ship GNU-only or support both.
- Decide whether to update local `main` to the rebased branch (currently `auto-continue-latest`) vs keep it as a legacy branch.

## P2

- Decide whether `rg` should be bundled for Windows/Linux distributions (currently only `apply_patch` is bundled in the x64 zips).

## Done

- Rebasing onto the latest upstream `origin/main`, then re-applying/augmenting the auto-continue feature + verbose defaults on top of upstream.
- Fixed TUI reliability after interrupts (`Esc` / "Conversation interrupted"): queued prompts submitted even if entered before SessionConfigured; interrupt clears stuck MCP startup running state.
- Made `Ctrl+C` behavior predictable when idle (exit) and when running (interrupt), without being blocked by MCP startup “running” state.
- Added a reproducible integration test for `--auto-continue` when the agent omits the `AUTO_MODE_NEXT=...` marker (should still enqueue and submit the follow-up prompt unless an explicit `stop` is present).
- Hardened `--auto-continue` flow across `/new` + resume/fork, and made `AUTO_MODE_NEXT=stop` behave as a temporary pause even when `TurnStarted` isn’t observed.
- Improved error handling so `--auto-continue` can enqueue on turn-ending `EventMsg::Error` (only when a turn is actually running).
- Made tool output fully visible by default (new `tui.show_full_tool_output`, no transcript ellipses for exec/MCP/patch failure output).
- Restarted the TUI event stream after crossterm EOF/error, emitting a redraw and adding a regression test.
- Added frame scheduler auto-respawn when the draw queue closes to prevent TUI redraw stalls.
- Recreate the TUI event stream if it ends unexpectedly, resuming the broker and scheduling a redraw.
- Make rollout persistence nonblocking for live event delivery to avoid UI stalls when the writer is backpressured.
- Skip cursor-position queries while the crossterm poller is active (pause broker for query; skip if in flight).
- Skip resume-time cursor queries unless the event stream is paused.
- Watchdog restarts the event stream after prolonged no-draw while streaming.
- Avoid blocking the runtime on the event broker lock (use try_lock in poll).
- Release broker lock while polling the event source (avoid pause/resume lock contention).
- Built, codesigned, and installed macOS `codex` + `apply_patch` (Homebrew prefix), and produced versioned Linux x64 + Windows x64 zip artifacts in `dist/`.
- YOLO now hard-enables shell/unified_exec/apply_patch/view_image/web_search with explicit full-access help text and disabled exec-policy enforcement.
- Auto-continue prompts now enforce 12+ tasks per turn and explicit scope expansion into tests/docs when needed.
- Centralized YOLO override handling and ensured it replaces conflicting config/CLI overrides for full-power behavior.
- TUI `--search` override now replaces conflicting `web_search` config entries, and CLI override precedence documented.
- Auto-continue guidance now warns against approval-loop stalls and aligns batch targets to 12-18 tasks.
- Auto-continue follow-up prompt now treats "Continue" as approval for pending required steps.
- Auto-continue approval semantics documented in README.
- TUI auto-continue flag help and docs mention approval semantics and max-turns cap.
- Added local build/install script for repeatable release builds, codesign, install, and cleanup.
- Added helper-script usage/logging notes in install docs and root README.
- Added `just build-install-local` target for repeatable local build/install.
- YOLO now force-resets shell environment policy overrides to ensure full env inheritance.
- YOLO CLI help text now mentions full env inheritance behavior.
- Auto-continue guidance now targets 12+ tasks per turn and at least three substantial deliverables.
- Auto-continue CLI/config docs now mention the 12-18 task batching target.
- YOLO docs now state that Codex applies no internal restrictions and relies on host security.
- YOLO overrides now force `features.exec_policy=false` to remove exec-policy gating.
- Deprecated `tools.web_search` is now ignored with a deprecation notice instead of breaking config load.
- Removed legacy web_search override usage in exec/tui; clarified deprecation docs and SDK comment.
- Removed `[tools].web_search` config plumbing; migrated config RPC tests to `web_search` mode.
- Removed legacy `[features].web_search` alias handling.
- Removed `tools_web_search_request` overrides from config harness.
- Added EventBroker pause/resume/paused-duration stats and watchdog logging for hang triage.
- Added macOS hang capture helper, SIGUSR1 diagnostics, and TUI hang debugging doc.
- Honored `tools.view_image`/profile overrides by gating the tool registry, with coverage tests.
