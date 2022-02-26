build:
    cargo build

testing: build
    cargo run -p lem-cli -- examples/println.lem

# todo: custom test runner
test: build
    cargo test

clean:
    cargo clean
    cd docs && 
    mdbook clean

# for whatever reason, this doesn't work if they're on seperate lines
serve-book:
    cd docs && mdbook serve

# todo: ci integration
release: dev-install
    cargo build --release

dev-install:
    cargo install rustfmt
    cargo install clippy
    cargo install mdbook

vercel-install:
    mkdir bin
    curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.15/mdbook-v0.4.15-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=bin
