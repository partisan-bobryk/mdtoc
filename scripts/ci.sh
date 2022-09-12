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
BUILD_TARGET=$1

cargo build --release --target $BUILD_TARGET --target-dir "$BUILD_PATH/$BUILD_TARGET"

ls -al "$BUILD_PATH"