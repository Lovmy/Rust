#!/bin/bash

echo Mise a jour ____________________________________________________________________________________________________________
cd ~/.cargo/bin
./rustup update

echo application ____________________________________________________________________________________________________________
cd ~/workspace/Rust/application
cargo clean
cargo build
cd ..

echo web_assembly ___________________________________________________________________________________________________________
cd ~/workspace/Rust/web_assembly
cargo clean
#cargo build --target wasm32-unknown-unknown --release
wasm-pack build
cd ..
