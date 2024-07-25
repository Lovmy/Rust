#!/bin/bash

echo application ____________________________________________________________________________________________________________
cd application
cargo clean
cargo build
cd ..

echo web_assembly ___________________________________________________________________________________________________________
cd web_assembly
cargo clean
#cargo build --target wasm32-unknown-unknown --release
#wasm-pack build
cd ..
