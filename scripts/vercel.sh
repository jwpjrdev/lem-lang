# install dependencies for mdbook to deploy on vercel

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
cd .cargo/bin/
./cargo install mdbook
