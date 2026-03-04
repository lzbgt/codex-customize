# Configuration

For basic configuration instructions, see [this documentation](https://developers.openai.com/codex/config-basic).

For advanced configuration instructions, see [this documentation](https://developers.openai.com/codex/config-advanced).

For a full configuration reference, see [this documentation](https://developers.openai.com/codex/config-reference).

## Connecting to MCP servers

Codex can connect to MCP servers configured in `~/.codex/config.toml`. See the configuration reference for the latest MCP server options:

- https://developers.openai.com/codex/config-reference

## Apps (Connectors)

Use `$` in the composer to insert a ChatGPT connector; the popover lists accessible
apps. The `/apps` command lists available and installed apps. Connected apps appear first
and are labeled as connected; others are marked as can be installed.

## Notify

Codex can run a notification hook when the agent finishes a turn. See the configuration reference for the latest notification settings:

- https://developers.openai.com/codex/config-reference

## JSON Schema

The generated JSON Schema for `config.toml` lives at `codex-rs/core/config.schema.json`.

## Legacy keys

Some legacy config keys are still accepted for compatibility but are deprecated:

- `[tools].web_search` and the top-level legacy alias `web_search_request` are deprecated. Prefer
  the top-level `web_search = "disabled" | "cached" | "live"` mode, or set
  `[features].web_search_request = true` if you need the raw tool toggle.

## CLI overrides

Codex applies `--config key=value` overrides after loading `~/.codex/config.toml`, so the CLI wins
over config defaults. In YOLO mode (`--yolo`), Codex replaces any conflicting overrides to force
full-power behavior (live web search, unrestricted tool enablement, and a fully inherited shell
environment with default excludes disabled).

When the TUI runs with `--auto-continue`, the follow-up "Continue" prompt also grants approval
for any previously requested required step unless the user explicitly declined. Use
`--auto-continue-max-turns N` to cap the number of turns.

## Notices

Codex stores "do not show again" flags for some UI prompts under the `[notice]` table.

Ctrl+C/Ctrl+D quitting uses a ~1 second double-press hint (`ctrl + c again to quit`).
