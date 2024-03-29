#!/bin/bash

# Exit script if you try to use an uninitialized variable.
set -o nounset
# Exit script if a statement returns a non-true return value.
set -o errexit
# Use the error status of the first failure, rather than that of the last item in a pipeline.
set -o pipefail
set -o xtrace

BUILD_PATH="bin"
BUILD_TARGET=$1
ARTIFACT_NAME="mdtoc-$BUILD_TARGET.tar.gz"

source "$HOME"/.cargo/env

rustup default $RUST_VERSION
rustup target add $BUILD_TARGET


# When building for linux distributions we need to specify a compiler
# via env variable and add a linker
if [ "$BUILD_TARGET" = "x86_64-unknown-linux-musl" ]; then
    TARGET_CC=x86_64-linux-musl-gcc
    brew install FiloSottile/musl-cross/musl-cross
fi

cargo build --release --target $BUILD_TARGET --target-dir "$BUILD_PATH"

# TODO Code Signing

# Package up
cd "$BUILD_PATH/$BUILD_TARGET/release"
chmod +x mdtoc
tar cfz $ARTIFACT_NAME mdtoc
cd -
mv "$BUILD_PATH/$BUILD_TARGET/release/$ARTIFACT_NAME" .
