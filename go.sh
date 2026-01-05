#!/bin/sh

# https://doc.rust-lang.org/nightly/rustc/platform-support.html

# Pour le statique :
# 	rustup target add x86_64-unknown-linux-musl
# 	sudo apt-get install -y musl-tools (Ubuntu)
#	sudo dnf install -y musl-gcc musl-devel (Fedora)
# 	cargo install cargo-vendor-openssl (si erreur utiliser plutot : openssl = { version = "*", features = ["vendored"] })

# Pour le NAS :
# 	rustup target add aarch64-unknown-linux-gnu
# 	sudo apt-get install -y gcc-aarch64-linux-gnu pkg-config (Ubuntu)
#	sudo dnf install -y gcc-aarch64-linux-gnu pkgconf-pkg-config (Fedora)
#	apk add --no-cache aarch64-linux-musl-gcc pkgconf (Alpine)

# Pour Windows ( à partir d'un Linux )
#	rustup target add x86_64-pc-windows-gnu
#	sudo apt-get install mingw-w64 (Ubuntu)
#	sudo dnf install -y mingw64-gcc mingw64-crt mingw64-headers (Fedora)
#	apk add --no-cache mingw-w64-gcc (Alpine)
#	Configurer le linker : créer/éditer ~/.cargo/config.toml :
#	[target.x86_64-pc-windows-gnu]
#	linker = "x86_64-w64-mingw32-gcc"

case "$1" in
	statique)
		cible=x86_64-unknown-linux-musl
		;;
	arm64)
		cible=aarch64-unknown-linux-gnu
		;;
	windows)
		cible=x86_64-pc-windows-gnu
		;;
	*)
		echo "$0 statique, arm64 ou windows. Si pas de paramètre compilation par défaut pour le système d'exploitation."
		;;
esac

echo Mise a jour ____________________________________________________________________________________________________________
cd ~/.cargo/bin
./rustup update
cargo install wasm-pack

echo application ____________________________________________________________________________________________________________
cd ~/workspace/Rust/application
cargo clean

case "$cible" in
	x86_64-unknown-linux-musl)
		OPENSSL_STATIC=1 PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target x86_64-unknown-linux-musl
		;;
	aarch64-unknown-linux-gnu)
		CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --release --target aarch64-unknown-linux-gnu
		;;
	x86_64-pc-windows-gnu)
		cargo build --release --target x86_64-pc-windows-gnu
		;;
	*)
		cargo build --release
		;;
esac
cd ..

#echo web_assembly ___________________________________________________________________________________________________________
cd ~/workspace/Rust/web_assembly
cargo clean
cargo build --target wasm32-unknown-unknown --release
#wasm-pack build
cd ..

rustup show
if [ -n "$cible" ]; then
	echo "Cible : $1."
else
	echo "Cible par défaut."
fi
ls -la application/target/$cible/release/
exit 0
