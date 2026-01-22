#!/usr/bin/env bash
set -euo pipefail

dry_run=0
assume_yes=0
keep_version=""

usage() {
  cat <<'EOF'
Usage: scripts/prune_build_outputs.sh [--version <git-describe>] [--dry-run] [--yes]

Prunes large local build outputs for this repo:
- Removes `codex-rs/target/` (local Rust build artifacts)
- Removes `.cache/` (if present)
- Removes `codex-cli/node_modules/.cache/` (if present)
- Keeps only the versioned x64 zip artifacts in `dist/` for the chosen version

Options:
  --version V  Keep only zips matching:
              dist/codex-linux-x64-V.zip and dist/codex-windows-x64-V.zip
              Defaults to `git describe --tags --always --dirty`.
  --dry-run    Print what would be removed, but do not delete
  --yes        Do not prompt
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      keep_version="${2:-}"
      shift 2
      ;;
    --dry-run)
      dry_run=1
      shift
      ;;
    --yes)
      assume_yes=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "${repo_root}"

if [[ ! -d "codex-rs" || ! -f "justfile" || ! -d ".git" ]]; then
  echo "Refusing to run: expected to be at the codex repo root (got: ${repo_root})" >&2
  exit 2
fi

if [[ -z "${keep_version}" ]]; then
  keep_version="$(git describe --tags --always --dirty 2>/dev/null || echo unknown)"
fi

linux_zip="codex-linux-x64-${keep_version}.zip"
windows_zip="codex-windows-x64-${keep_version}.zip"

echo "Repo: ${repo_root}"
echo "Keeping dist zips:"
echo "  - dist/${linux_zip}"
echo "  - dist/${windows_zip}"
echo ""

declare -a remove_paths=()

if [[ -d "codex-rs/target" ]]; then
  remove_paths+=("codex-rs/target")
fi
if [[ -d ".cache" ]]; then
  remove_paths+=(".cache")
fi
if [[ -d "codex-cli/node_modules/.cache" ]]; then
  remove_paths+=("codex-cli/node_modules/.cache")
fi

if [[ -d "dist" ]]; then
  shopt -s nullglob
  for f in dist/*; do
    base="$(basename "$f")"
    if [[ "${base}" == "${linux_zip}" || "${base}" == "${windows_zip}" ]]; then
      continue
    fi
    remove_paths+=("dist/${base}")
  done
  shopt -u nullglob
fi

if [[ ${#remove_paths[@]} -eq 0 ]]; then
  echo "Nothing to prune."
  exit 0
fi

echo "Will remove:"
for p in "${remove_paths[@]}"; do
  echo "  - ${p}"
done

if [[ "${dry_run}" -eq 1 ]]; then
  echo ""
  echo "Dry run: no changes made."
  exit 0
fi

if [[ "${assume_yes}" -ne 1 ]]; then
  echo -n "Proceed? [y/N] "
  read -r reply
  if [[ "${reply}" != "y" && "${reply}" != "Y" ]]; then
    echo "Aborted."
    exit 1
  fi
fi

for p in "${remove_paths[@]}"; do
  rm -rf -- "${p}"
done

echo "Prune complete."

