#!/usr/bin/env bash
set -euo pipefail

readonly MIN_INTERVAL_SECONDS=120
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
readonly STATE_FILE="${TMPDIR:-/tmp}/codex-atcoder-cargo-compete-new-last-start"
readonly LOCK_DIR="${TMPDIR:-/tmp}/codex-atcoder-cargo-compete-new.lock"

usage() {
    echo "Usage: $0 <contest> [<contest> ...]"
    echo "Runs cargo_compete_new.sh with at least ${MIN_INTERVAL_SECONDS} seconds between AtCoder request starts."
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    usage
    exit 0
fi
if [[ "$#" -eq 0 ]]; then
    usage >&2
    exit 2
fi

while ! mkdir "$LOCK_DIR" 2>/dev/null; do
    echo "Waiting for another paced AtCoder retrieval to finish..." >&2
    sleep 1
done
cleanup() {
    rmdir "$LOCK_DIR" 2>/dev/null || true
}
trap cleanup EXIT INT TERM

for contest in "$@"; do
    if [[ ! "$contest" =~ ^[A-Za-z0-9_-]+$ ]]; then
        echo "Error: Invalid contest directory name: ${contest}" >&2
        exit 2
    fi

    now=$(date +%s)
    last=0
    if [[ -f "$STATE_FILE" ]]; then
        read -r last < "$STATE_FILE" || last=0
    fi
    if [[ "$last" =~ ^[0-9]+$ ]]; then
        wait_seconds=$((last + MIN_INTERVAL_SECONDS - now))
        if (( wait_seconds > 0 )); then
            echo "Waiting ${wait_seconds}s before AtCoder retrieval for ${contest} (minimum interval: ${MIN_INTERVAL_SECONDS}s)." >&2
            sleep "$wait_seconds"
        fi
    fi

    date +%s > "$STATE_FILE"
    if [[ -d "$ROOT/src/contest/$contest" ]]; then
        echo "Removing existing contest directory for ${contest}." >&2
        rm -rf -- "$ROOT/src/contest/$contest"
    fi
    echo "Running spaced AtCoder retrieval for ${contest}." >&2
    bash "$ROOT/.devcontainer/cargo_compete_new.sh" "$contest"
done
