#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="${ROOT_DIR}/build/logs"
TS="$(date +%Y%m%d_%H%M%S)"

usage() {
  cat <<'EOF'
Usage: scripts/build_install_local.sh

Builds codex-cli + apply_patch (release), codesigns on macOS, installs to a local prefix,
and prunes build outputs. Logs are written under build/logs.

Env:
  CODEX_INSTALL_PREFIX  Override install prefix (default: /opt/homebrew or /usr/local)
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

mkdir -p "${LOG_DIR}"

log() {
  echo "[$(date +%H:%M:%S)] $*"
}

build_release() {
  local crate="$1"
  local log_file="${LOG_DIR}/cargo_build_${crate}_release_${TS}.log"
  log "Building ${crate} (release) -> ${log_file}"
  (cd "${ROOT_DIR}/codex-rs" && cargo build -p "${crate}" --release > "${log_file}" 2>&1)
}

codesign_binaries() {
  local log_file="${LOG_DIR}/codesign_${TS}.log"
  if [[ "$(uname -s)" != "Darwin" ]]; then
    log "Skipping codesign (not macOS)"
    return 0
  fi
  if ! command -v codesign >/dev/null 2>&1; then
    log "Skipping codesign (codesign not found)"
    return 0
  fi
  log "Codesigning codex/apply_patch -> ${log_file}"
  codesign --force --sign - \
    "${ROOT_DIR}/codex-rs/target/release/codex" \
    "${ROOT_DIR}/codex-rs/target/release/apply_patch" > "${log_file}" 2>&1
}

install_binaries() {
  local prefix="${CODEX_INSTALL_PREFIX:-}"
  if [[ -z "${prefix}" ]]; then
    if [[ -d "/opt/homebrew/bin" ]]; then
      prefix="/opt/homebrew"
    else
      prefix="/usr/local"
    fi
  fi
  local log_file="${LOG_DIR}/install_binaries_${TS}.log"
  log "Installing to ${prefix}/bin -> ${log_file}"
  install -m 0755 "${ROOT_DIR}/codex-rs/target/release/codex" "${prefix}/bin/codex" \
    > "${log_file}" 2>&1
  install -m 0755 "${ROOT_DIR}/codex-rs/target/release/apply_patch" "${prefix}/bin/apply_patch" \
    >> "${log_file}" 2>&1
}

prune_build_outputs() {
  local log_file="${LOG_DIR}/prune_build_outputs_${TS}.log"
  log "Pruning build outputs -> ${log_file}"
  "${ROOT_DIR}/scripts/prune_build_outputs.sh" --yes > "${log_file}" 2>&1
}

build_release "codex-cli"
build_release "codex-apply-patch"
codesign_binaries
install_binaries
prune_build_outputs

log "Done."
