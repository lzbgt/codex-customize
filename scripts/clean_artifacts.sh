#!/usr/bin/env bash
set -euo pipefail

minutes=6
dry_run=0
assume_yes=0

usage() {
  cat <<'EOF'
Usage: scripts/clean_artifacts.sh [--minutes N] [--dry-run] [--yes]

Deletes common build/cache artifacts older than N minutes.

Defaults:
  --minutes 6

Options:
  --minutes N   Age threshold in minutes (integer, >0)
  --dry-run     Print what would be deleted, but do not delete
  --yes         Do not prompt before deleting
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --minutes)
      minutes="${2:-}"
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

if [[ -z "${minutes}" ]] || ! [[ "${minutes}" =~ ^[0-9]+$ ]] || [[ "${minutes}" -le 0 ]]; then
  echo "--minutes must be a positive integer (got: '${minutes}')" >&2
  exit 2
fi

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "${repo_root}"

targets=(
  ".cache"
  "codex-rs/target"
  "codex-cli/node_modules/.cache"
)

declare -a candidates=()

for dir in "${targets[@]}"; do
  if [[ -d "${dir}" ]]; then
    candidates+=("${dir}")
  fi
done

if [[ ${#candidates[@]} -eq 0 ]]; then
  echo "No known artifact directories exist in ${repo_root}."
  exit 0
fi

echo "Repo: ${repo_root}"
echo "Cleaning artifacts older than ${minutes} minutes in:"
for dir in "${candidates[@]}"; do
  echo "  - ${dir}"
done

if [[ "${dry_run}" -eq 1 ]]; then
  echo ""
  echo "Dry run: showing deletions only."
fi

echo ""

to_delete_count=0
while IFS= read -r -d '' path; do
  ((to_delete_count+=1))
  printf '%s\0' "${path}"
done < <(
  for dir in "${candidates[@]}"; do
    # `-mmin` is supported on both BSD/macOS and GNU find.
    find "${dir}" -type f -mmin "+${minutes}" -print0 2>/dev/null || true
  done
)

if [[ "${to_delete_count}" -eq 0 ]]; then
  echo "Nothing to delete."
  exit 0
fi

echo "Found ${to_delete_count} files to delete."

if [[ "${dry_run}" -eq 1 ]]; then
  echo "Run again without --dry-run to delete."
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

deleted=0
while IFS= read -r -d '' file; do
  rm -f -- "${file}"
  ((deleted+=1))
done < <(
  for dir in "${candidates[@]}"; do
    find "${dir}" -type f -mmin "+${minutes}" -print0 2>/dev/null || true
  done
)

# Remove empty directories bottom-up.
for dir in "${candidates[@]}"; do
  find "${dir}" -type d -empty -delete 2>/dev/null || true
done

echo "Deleted ${deleted} files."

