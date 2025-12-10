#!/bin/bash

# Pour le statique :
#	rustup target add x86_64-unknown-linux-musl
#	sudo apt-get install -y musl-tools
#	cargo install cargo-vendor-openssl (si erreur utiliser plutot : openssl = { version = "*", features = ["vendored"] })

# Pour le NAS :
#	rustup target add aarch64-unknown-linux-gnu
#	sudo apt-get install -y gcc-aarch64-linux-gnu pkg-config

echo Mise a jour ____________________________________________________________________________________________________________
#cd ~/.cargo/bin
#./rustup update
cargo install wasm-pack

echo application ____________________________________________________________________________________________________________
cd ~/workspace/Rust/application
cargo clean
OPENSSL_STATIC=1 PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target x86_64-unknown-linux-musl
#CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --target aarch64-unknown-linux-gnu
cd ..

echo web_assembly ___________________________________________________________________________________________________________
cd ~/workspace/Rust/web_assembly
cargo clean
cargo build --target wasm32-unknown-unknown --release
#wasm-pack build
cd ..
git config pull.rebase true