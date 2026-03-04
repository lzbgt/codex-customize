#!/usr/bin/env bash
set -euo pipefail

pid="${1:-}"
duration="${2:-5}"

usage() {
  cat <<'USAGE'
Usage: scripts/capture_tui_sample.sh <pid> [duration_seconds]

Captures a macOS `sample` snapshot for the running Codex TUI process.

Environment:
  OUT_DIR   Directory for logs (default: build/logs)
USAGE
}

if [[ -z "${pid}" ]]; then
  usage >&2
  exit 2
fi

if ! [[ "${duration}" =~ ^[0-9]+$ ]]; then
  echo "duration_seconds must be an integer (got: '${duration}')" >&2
  exit 2
fi

if ! command -v sample >/dev/null 2>&1; then
  echo "sample tool not found (macOS only)." >&2
  exit 127
fi

if ! kill -0 "${pid}" 2>/dev/null; then
  echo "process ${pid} not running or not accessible" >&2
  exit 1
fi

out_dir="${OUT_DIR:-build/logs}"
mkdir -p "${out_dir}"

ts=$(date +%Y%m%d_%H%M%S)
out_file="${out_dir}/sample_codex_${pid}_${ts}.txt"

echo "Sampling pid ${pid} for ${duration}s -> ${out_file}"
# shellcheck disable=SC2086
sample "${pid}" "${duration}" -file "${out_file}"

echo "Wrote ${out_file}"
