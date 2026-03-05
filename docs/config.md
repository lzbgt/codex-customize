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

Some legacy config keys are deprecated or removed:

- `[tools].web_search` is deprecated and ignored. Use the top-level
  `web_search = "disabled" | "cached" | "live"` mode instead. If you need the raw tool toggle,
  set `[features].web_search_request = true`.
- `[features].web_search` is deprecated and ignored. Use `[features].web_search_request`.
  To enable the built-in web search tool, set `web_search = "live" | "cached" | "disabled"`.
- Unknown `[features]` keys are ignored with a warning. Check the configuration reference for
  supported feature flags.

## Config diagnostics

Use the CLI to inspect which configuration layers are active and to surface deprecated keys:

- `codex config layers` shows the active config layers (highest precedence first), including
  disabled layers and any deprecated keys detected per layer.
- `codex config warnings` summarizes deprecated keys and unknown `[features]` entries.
- Use `--json` with either subcommand to emit machine-readable output.
  The JSON payload for `layers` includes `source`, `version`, `enabled`, `disabled_reason`,
  `precedence` (0 = highest), `deprecated_keys` for each layer, and `layer_count`.
  The JSON payload for `warnings` includes `has_warnings`, `deprecated_count`,
  `warnings_count`, and per-key `counts`.
  Both JSON payloads include `profile` and `cwd` metadata for context.

Pass `--profile` to target a specific profile or `--cwd` to resolve project layers for a
different working directory.

## CLI overrides

Codex applies `--config key=value` overrides after loading `~/.codex/config.toml`, so the CLI wins
over config defaults. In YOLO mode (`--yolo`), Codex replaces any conflicting overrides to force
full-power behavior (live web search, unrestricted tool enablement, `features.exec_policy=false`,
and a fully inherited shell environment with default excludes disabled). Codex applies no internal
restrictions in YOLO; the user accepts the risk and relies on the host security model.
Setting `profile = "yolo"` in `config.toml` applies the same full-power overrides as `--yolo`,
even if you do not define `[profiles.yolo]`. If you do define `[profiles.yolo]`, any additional
settings are honored, but YOLO still forces the unrestricted defaults. This also bypasses managed
requirements/allowlists and managed config layers that would otherwise constrain approval policy,
sandboxing, or MCP servers.

When the TUI runs with `--auto-continue`, the follow-up "Continue" prompt also grants approval
for any previously requested required step unless the user explicitly declined. The follow-up
prompt targets 48-72 related tasks per turn (minimum 48). Use `--auto-continue-max-turns N` to cap
the number of turns.

## Notices

Codex stores "do not show again" flags for some UI prompts under the `[notice]` table.

Ctrl+C/Ctrl+D quitting uses a ~1 second double-press hint (`ctrl + c again to quit`).
