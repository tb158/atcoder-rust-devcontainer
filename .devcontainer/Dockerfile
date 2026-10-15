FROM rust:1.89.0-bookworm

ENV CLAUDE_CODE_AUTO_COMPACT_WINDOW=200000
ENV PATH="/opt/kimi-code/bin:${PATH}"

RUN apt-get update && \
    apt-get install -y nodejs npm curl && \
    npm install -g @anthropic-ai/claude-code && \
    npm install -g @openai/codex && \
    curl -fsSL https://code.kimi.com/kimi-code/install.sh | env KIMI_INSTALL_DIR=/opt/kimi-code KIMI_NO_MODIFY_PATH=1 bash && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    rustup toolchain install 1.89.0 && \
    rustup component add rustfmt clippy && \
    # Analysis-only toolchain for rust-analyzer (see rust-analyzer.cargo.extraEnv
    # in .vscode/settings.json). The RA extension tracks the latest stable, and
    # against the 1.89.0 sysroot it fails to resolve std macros (`vec!`,
    # `println!`, `write!`) and alloc-gated methods (`Vec::push`), which surfaces
    # as bogus E0308s. Builds and submissions still use 1.89.0 (judge parity).
    # rustfmt is needed here too: rust-analyzer spawns rustfmt with the env from
    # rust-analyzer.cargo.extraEnv (RUSTUP_TOOLCHAIN=stable), so "Format Document"
    # silently fails unless the stable toolchain carries the component.
    rustup toolchain install stable --profile minimal --component rust-src --component rustfmt && \
    # cargo install cargo-compete --locked && \
    # tb158's fork (from qryxip/cargo-compete via slow-cat), distributed under MIT.
    cargo install --git https://github.com/tb158/cargo-compete --branch master --locked && \
    # cargo install --path /workspaces/atcoder-rust-devcontainer/cargo-compete --locked &&
    cargo install cargo-snippet --features="binaries" && \
    cargo install rustlings

COPY ./entrypoint.sh ./cargo_compete_new.sh ./create_gitkeep_in_testcases.sh ./cargo_snippet.sh /usr/local/bin/

RUN chmod +x /usr/local/bin/entrypoint.sh /usr/local/bin/cargo_compete_new.sh /usr/local/bin/create_gitkeep_in_testcases.sh /usr/local/bin/cargo_snippet.sh
