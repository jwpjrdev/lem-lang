# Install dependencies for mdbook to deploy on vercel

# This is better than being in the justfile because if this was in there, Vercel would have to
# 1. install cargo (and by extension rust),
# 2. compile just,
# 3. and then finally run these commands

mkdir bin
curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.15/mdbook-v0.4.15-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=bin
