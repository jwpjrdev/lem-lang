# grcov
rustup default nightly
cargo install grcov

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

# commitizen
npm i -g commitizen
commitizen init cz-conventional-changelog --save-dev --save-exact

# rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy
