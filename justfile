build:
    cargo build

# just example strings
set positional-arguments
@example file: build
    cargo run -- examples/$1.lem

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

# rustup default nightly
# rustup component add rustfmt --toolchain nightly
dev-install:
    rustup component add clippy-preview
    cargo install mdbook
