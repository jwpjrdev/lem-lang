build:
    cargo build

testing: build
    cargo run -p lem-cli -- examples/println.lem

# todo: custom test runner
test: build
    cargo test

clean:
    cargo clean
    cd docs && mdbook clean

# for whatever reason, this doesn't work if they're on seperate lines
serve-book:
    mdbook serve docs/

# todo: ci integration
release: dev-install
    cargo build --release

dev-install:
# rustup default nightly
# rustup component add rustfmt --toolchain nightly
    cargo install clippy
    cargo install mdbook
