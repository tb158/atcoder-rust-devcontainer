#!/bin/bash

# This script generates Rust code snippets for Visual Studio Code in the context of Competitive Programming.

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
