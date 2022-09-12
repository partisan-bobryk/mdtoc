#!/bin/bash

# Exit script if you try to use an uninitialized variable.
set -o nounset
# Exit script if a statement returns a non-true return value.
set -o errexit
# Use the error status of the first failure, rather than that of the last item in a pipeline.
set -o pipefail
set -o xtrace

source "$HOME"/.cargo/env
if [ "$RUST_VERSION" != "stable" ]; then
	rustup toolchain install "$RUST_VERSION"
	rustup default "$RUST_VERSION"
fi

BUILD_PATH="bin"
BUILD_TARGET=$1

# When building for linux distributions we need to specify a compiler
# via env variable
if [ "$BUILD_TARGET" = "x86_64-unknown-linux-musl" ]; then
    TARGET_CC=x86_64-linux-musl-gcc
fi

cargo build --release --target $BUILD_TARGET --target-dir "$BUILD_PATH/$BUILD_TARGET"

ls -al "$BUILD_PATH"