#!/usr/bin/env bash

# Exit script if you try to use an uninitialized variable.
set -o nounset

# Exit script if a statement returns a non-true return value.
set -o errexit

# Use the error status of the first failure, rather than that of the last item in a pipeline.
set -o pipefail

set -o xtrace

RUST_VERSION=stable
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
chmod +x rustup.sh
./rustup.sh -y

source "$HOME"/.cargo/env

if [ "$RUST_VERSION" != "stable" ]; then
	rustup toolchain install "$RUST_VERSION"
	rustup default "$RUST_VERSION"
fi

rustup target install $MACOS_ARM
rustup target install $MACOS_INTEL