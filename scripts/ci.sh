#!/bin/bash

BUILD_PATH="bin"
MACOS_ARM=aarch64-apple-darwin
MACOS_INTEL=x86_64-apple-darwin

rustup target install $MACOS_ARM
rustup target install $MACOS_INTEL

cargo build --release --target $MACOS_ARM --target-dir "$BUILD_PATH/$MACOS_ARM"
cargo build --release --target $MACOS_INTEL --target-dir "$BUILD_PATH/$MACOS_INTEL"

ls -al "$BUILD_PATH"