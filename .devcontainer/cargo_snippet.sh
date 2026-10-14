#!/bin/bash

# This script generates Rust code snippets for Visual Studio Code in the context of Competitive Programming.
#
# Usage:
#   cargo_snippet.sh             generate .vscode/rust.code-snippets, then offer to commit the
#                                snippet sources and push them to origin/main (asks y/N first;
#                                asks again during ABC hours, Sat 21:00-22:40 JST)
#   cargo_snippet.sh --no-push   generate only (used by entrypoint.sh at container creation)
#
# Declining, or running off main, still leaves freshly generated snippets; nothing is committed
# unless the push is confirmed.

PUSH=1
case "${1:-}" in
    "") ;;
    --no-push) PUSH=0 ;;
    *)
        echo "Usage: cargo_snippet.sh [--no-push]" >&2
        exit 2
        ;;
esac

# Files that make up the published snippets: the library, its generated snippets, and the
# hand-written / postfix snippets. Only these paths are committed.
SNIPPET_PATHS=(src/lib .vscode/rust.code-snippets .vscode/my-hand-rust.code-snippets .vscode/settings.json)

# Sharing code during a contest is against AtCoder's rules. Rather than querying AtCoder,
# use the usual ABC slot (Saturday 21:00-22:40 JST) and ask before pushing inside it.
contest_guard() {
    local now
    now=$(TZ=Asia/Tokyo date +%u%H%M)  # e.g. 62130 = Saturday 21:30
    if (( now >= 62100 && now < 62240 )); then
        echo "Warning: it is ABC time (Sat 21:00-22:40 JST). Do not publish code during a contest." >&2
        read -r -p "Push anyway? [y/N] " answer
        [[ "$answer" == [yY] ]] || { echo "Aborted." >&2; return 1; }
    fi
    return 0
}

# Public repository that holds src/lib. Every generated snippet body starts with this URL so
# that submitted code carries the reference link AtCoder's generative-AI rule asks for
# (code written with AI before a contest must be published beforehand and its URL attached
# when copied). Same form as the AHC environment (tb158/ahc-lib-rs).
LIB_URL="https://github.com/tb158/atcoder-rust-devcontainer"

# Navigate to the lib directory within the workspace folder
cd $WORKSPACE_FOLDER/src/lib

# Run the tests silently. Exit with an error if tests fail.
if ! cargo test &>/dev/null; then
    echo "Error: Tests failed."
    exit 1
fi

# Generate Rust code snippets using cargo-snippet, then modify the output with sed to
#   1. add a scope for Rust
#   2. drop `pub` / `pub(crate)` from item declarations. A handful of shared helpers
#      (to_i64, 半開, 累乗_mod, 無向グラフ, ...) must stay visible for the lib's own
#      cross-module `use crate::...`, but snippets are pasted into a single solution file
#      where the visibility modifier is pure noise.
#   3. insert $LIB_URL as a comment on the first line of every snippet body
# and finally, save the modified output to a rust.code-snippets file in the .vscode directory of the workspace folder
cargo snippet -t vscode \
    | sed -r "s/\"prefix\"/\"scope\": \"rust\",\n    \"prefix\"/" \
    | sed -r "s/^(      \")( *)pub(\([a-z]+\))? /\1\2/" \
    | sed -r 's|^(\s*)"body": \[$|\1"body": [\n\1  "// '"$LIB_URL"'",|' \
    > $WORKSPACE_FOLDER/.vscode/rust.code-snippets
echo "Snippets generated."

# Zed snippet generation is DISABLED: Zed (v1.7.x) crashes slicing multibyte completion
# labels (rust.rs:496 "begin > end"), and the crash fires on any completion popup, so the
# generated ~/.config/zed/snippets/rust.json made the editor unusable. The conversion script
# (snippets_to_zed.py) is kept for when the upstream bug is fixed; re-enable the block below
# to restore Zed snippets. VS Code snippet generation above is unaffected.
#
# ZED_SNIPPETS_DIR="${HOME}/.config/zed/snippets"
# mkdir -p "$ZED_SNIPPETS_DIR"
# python3 "$WORKSPACE_FOLDER/.devcontainer/snippets_to_zed.py" \
#     "$ZED_SNIPPETS_DIR/rust.json" \
#     "$WORKSPACE_FOLDER/.vscode/rust.code-snippets" \
#     "$WORKSPACE_FOLDER/.vscode/my-hand-rust.code-snippets"
# echo "Zed snippets generated."

# --- Publish ---
(( PUSH )) || exit 0
cd "$WORKSPACE_FOLDER" || exit 1

branch=$(git branch --show-current)
if [[ "$branch" != "main" ]]; then
    echo "Not pushed: publish from main (current branch: ${branch:-detached HEAD})."
    exit 0
fi

changes=$(git status --porcelain -- "${SNIPPET_PATHS[@]}")
git fetch -q origin main || { echo "Not pushed: could not fetch origin." >&2; exit 1; }
unpushed=$(git rev-list origin/main..HEAD)
if [[ -z "$changes" && -z "$unpushed" ]]; then
    echo "Nothing to push."
    exit 0
fi

echo
if [[ -n "$changes" ]]; then
    echo "Snippet changes to commit:"
    git status --short -- "${SNIPPET_PATHS[@]}"
fi
if [[ -n "$unpushed" ]]; then
    echo "Commits not yet on origin/main:"
    git log --oneline origin/main..HEAD
fi
read -r -p "Commit and push to origin/main? [y/N] " answer
if [[ "$answer" != [yY] ]]; then
    echo "Not pushed; nothing was committed."
    exit 0
fi
contest_guard || { echo "Not pushed; nothing was committed."; exit 0; }

if [[ -n "$changes" ]]; then
    # Commit only the snippet paths, even if other changes are staged.
    git add -- "${SNIPPET_PATHS[@]}" || exit 1
    git commit -q -m "Update snippets" -- "${SNIPPET_PATHS[@]}" || exit 1
fi
git push origin main
