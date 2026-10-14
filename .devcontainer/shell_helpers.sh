#!/bin/bash

# Kimi Code CLI: persist config/sessions under .vscode/kimi-code (like .claude / .codex).
if [[ -n "${WORKSPACE_FOLDER:-}" ]]; then
    export KIMI_CODE_HOME="${WORKSPACE_FOLDER}/.vscode/kimi-code"
    mkdir -p "$KIMI_CODE_HOME"
fi

abc_cd() {
    local contest_file="${WORKSPACE_FOLDER:?WORKSPACE_FOLDER is not set}/.abc_last_contest"
    if [[ ! -f "$contest_file" ]]; then
        echo "Error: No contest directory has been recorded yet." >&2
        return 1
    fi

    local contest_dir
    contest_dir=$(<"$contest_file")
    if [[ ! -d "$contest_dir" ]]; then
        echo "Error: Contest directory not found: $contest_dir" >&2
        return 1
    fi

    cd "$contest_dir"
}

function cargo_compete_new.sh {
    bash "${WORKSPACE_FOLDER:?WORKSPACE_FOLDER is not set}/.devcontainer/cargo_compete_new.sh" "$@" || return
    abc_cd
}

# The image bakes a copy into /usr/local/bin at build time; always run the workspace
# copy so edits (e.g. the reference URL line) apply without a container rebuild.
function cargo_snippet.sh {
    bash "${WORKSPACE_FOLDER:?WORKSPACE_FOLDER is not set}/.devcontainer/cargo_snippet.sh" "$@"
}
