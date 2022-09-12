#!/bin/bash

# Exit script if you try to use an uninitialized variable.
set -o nounset

# Exit script if a statement returns a non-true return value.
set -o errexit

# Use the error status of the first failure, rather than that of the last item in a pipeline.
set -o pipefail

set -o xtrace

source "$HOME"/.cargo/env

BUILD_PATH="bin"
MACOS_ARM=aarch64-apple-darwin
MACOS_INTEL=x86_64-apple-darwin

cargo build --release --target $MACOS_ARM --target-dir "$BUILD_PATH/$MACOS_ARM"
cargo build --release --target $MACOS_INTEL --target-dir "$BUILD_PATH/$MACOS_INTEL"

ls -al "$BUILD_PATH"