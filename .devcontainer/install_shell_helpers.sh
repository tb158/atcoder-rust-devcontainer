#!/bin/bash

set -e

# Put the Rust toolchain back on PATH for login shells. The base image exports
# /usr/local/cargo/bin via Docker ENV, but /etc/profile resets PATH for login
# shells (e.g. Zed's integrated terminal), so `cargo` ends up "command not found".
# /etc/profile.d/*.sh is sourced by login shells, restoring it.
cat > /etc/profile.d/cargo-path.sh <<'PROFILE_EOF'
# Ensure the Rust toolchain (cargo/rustc via rustup) is on PATH for login shells.
cargo_bin="${CARGO_HOME:-/usr/local/cargo}/bin"
case ":${PATH}:" in
    *":${cargo_bin}:"*) ;;
    *) export PATH="${cargo_bin}:${PATH}" ;;
esac
unset cargo_bin

# Kimi Code CLI (installed to /opt/kimi-code by the devcontainer image).
kimi_bin="/opt/kimi-code/bin"
case ":${PATH}:" in
    *":${kimi_bin}:"*) ;;
    *) export PATH="${kimi_bin}:${PATH}" ;;
esac
unset kimi_bin
PROFILE_EOF

bashrc_path="/root/.bashrc"
block_start="# >>> workspace shell helpers >>>"
block_end="# <<< workspace shell helpers <<<"

if grep -Fq "$block_start" "$bashrc_path"; then
    sed -i "/$block_start/,/$block_end/d" "$bashrc_path"
fi

legacy_block_start="# >>> ahc_cd >>>"
legacy_block_end="# <<< ahc_cd <<<"
if grep -Fq "$legacy_block_start" "$bashrc_path"; then
    sed -i "/$legacy_block_start/,/$legacy_block_end/d" "$bashrc_path"
fi

old_contest_file="$WORKSPACE_FOLDER/.ahc_last_contest"
contest_file="$WORKSPACE_FOLDER/.abc_last_contest"
if [[ -f "$old_contest_file" && ! -f "$contest_file" ]]; then
    mv "$old_contest_file" "$contest_file"
fi

cat >> "$bashrc_path" <<'EOF'

# >>> workspace shell helpers >>>
if [[ -n "${WORKSPACE_FOLDER:-}" && -f "$WORKSPACE_FOLDER/.devcontainer/shell_helpers.sh" ]]; then
    source "$WORKSPACE_FOLDER/.devcontainer/shell_helpers.sh"
fi
# <<< workspace shell helpers <<<
EOF
