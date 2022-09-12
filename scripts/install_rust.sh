#!/usr/bin/env bash
# Exit script if you try to use an uninitialized variable.
set -o nounset
# Exit script if a statement returns a non-true return value.
set -o errexit
# Use the error status of the first failure, rather than that of the last item in a pipeline.
set -o pipefail
set -o xtrace

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
chmod +x rustup.sh
./rustup.sh -y

source "$HOME"/.cargo/env

rustup toolchain install "$RUST_VERSION"
rustup default $RUST_VERSION


# Install Linux Musl linker
brew install FiloSottile/musl-cross/musl-cross

# Install targets
rustup target install aarch64-apple-darwin
rustup target install x86_64-apple-darwin
rustup target install x86_64-unknown-linux-musl