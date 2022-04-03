# Contributing to Lem

## Getting Started

### Local Environment

1. Ensure that Rust, `cargo`, and `rustup` are installed on your system
2. Run `cargo install just` to install Just
3. Run `just dev-install` to install dependencies
4. Run `just serve-book` to start the documentation server
5. Visit `http://localhost:3000` to see the documentation book

### Cloud Environment

Click [this](https://gitpod.io/#https://github.com/jwpjrdev/lem-lang) link to open Lem in Gitpod.

## Committing Changes

This project strictly follows conventional commits.

Please restrict scope to any of the following: `lexer`, `parser`, `interpreter`, or `cli`. Documentation changes do not use a scope, and instead use the `docs` type.

Please also run the following before committing or creating a pull request:
- `just test`
- `just format`
