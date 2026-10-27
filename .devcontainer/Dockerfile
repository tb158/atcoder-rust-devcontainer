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
    rustup component add rustfmt clippy rust-src && \
    rustup toolchain install 1.99.1 --profile minimal --component rust-analyzer && \
    # cargo install cargo-compete --locked && \
    # tb158's fork (from qryxip/cargo-compete via slow-cat), distributed under MIT.
    cargo install --git https://github.com/tb158/cargo-compete --branch master --locked && \
    # cargo install --path /workspaces/atcoder-rust-devcontainer/cargo-compete --locked &&
    cargo install cargo-snippet --features="binaries" && \
    cargo install rustlings

COPY ./entrypoint.sh ./cargo_compete_new.sh ./create_gitkeep_in_testcases.sh ./cargo_snippet.sh /usr/local/bin/

RUN chmod +x /usr/local/bin/entrypoint.sh /usr/local/bin/cargo_compete_new.sh /usr/local/bin/create_gitkeep_in_testcases.sh /usr/local/bin/cargo_snippet.sh

COPY ./ra-std-prelude-macros.rs /tmp/

RUN mkdir -p /opt/ra-sysroot-src && \
    cp -r "$(rustc +1.89.0 --print sysroot)/lib/rustlib/src/rust/library" /opt/ra-sysroot-src/ && \
    cat /tmp/ra-std-prelude-macros.rs >> /opt/ra-sysroot-src/library/std/src/prelude/v1.rs && \
    rm /tmp/ra-std-prelude-macros.rs && \
    ln -s "$(rustup which --toolchain 1.99.1 rust-analyzer)" /usr/local/bin/rust-analyzer-ide
