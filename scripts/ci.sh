#!/bin/bash
RUST_VERSION=stable
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
chmod +x rustup.sh
./rustup.sh -y

source "$HOME"/.cargo/env

if [ "$RUST_VERSION" != "stable" ]; then
	rustup toolchain install "$RUST_VERSION"
	rustup default "$RUST_VERSION"
fi

BUILD_PATH="bin"
MACOS_ARM=aarch64-apple-darwin
MACOS_INTEL=x86_64-apple-darwin

rustup target install $MACOS_ARM
rustup target install $MACOS_INTEL

cargo build --release --target $MACOS_ARM --target-dir "$BUILD_PATH/$MACOS_ARM"
cargo build --release --target $MACOS_INTEL --target-dir "$BUILD_PATH/$MACOS_INTEL"

ls -al "$BUILD_PATH"