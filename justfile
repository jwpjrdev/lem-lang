build:
    cargo build

test: build
    cargo test
    # custom test runner

release: dev-install
    cargo build --release
    # todo: ci integration

dev-install:
    cargo install rustfmt
    cargo install clippy
    cargo install mdbook

vercel-install:
    mkdir bin
    curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.15/mdbook-v0.4.15-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=bin
