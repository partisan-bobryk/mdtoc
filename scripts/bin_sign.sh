#!/bin/bash

# To test this executable run
# ./scripts/bin_sign.sh aarch64-apple-darwin
# ./scripts/bin_sign.sh x86_64-apple-darwin
# ./scripts/bin_sign.sh x86_64-unknown-linux-musl

# Exit script if you try to use an uninitialized variable.
set -o nounset
# Exit script if a statement returns a non-true return value.
set -o errexit
# Use the error status of the first failure, rather than that of the last item in a pipeline.
set -o pipefail
set -o xtrace

BUILD_TARGET=$1
BUILD_PATH="bin"
BIN_NAME="mdtoc"
BIN_DIR="$BUILD_PATH/$BUILD_TARGET/release"
BIN_LOCATION="$BIN_DIR/$BIN_NAME"

sign_linux () {
    local bin=$1
    echo 'Signing Linux Binary...'
    local bin_sha=$(shasum -a 512 $bin | awk '{print $1}')
    # TODO: For linux use gpg to sign the binary and attach a .sig file for validation
    echo $bin_sha
    return 0
}

sign_darwin () {
    local bin=$1
    echo 'Signing Darwin Binary...'
    codesign --timestamp --options=runtime -s "Developer ID Application" -i "studio.revent.mdtoc" $bin
}

sign () {
    local arch=$1
    local bin_location=$2

    if [ "$arch" = "x86_64-unknown-linux-musl" ]; then
        sign_linux $bin_location
    else
        sign_darwin $bin_location
    fi

    if [ "$?" -gt 0 ]; then
        echo 'Failed!'
    else
        echo 'Complete!'
    fi
}


sign $BUILD_TARGET $BIN_LOCATION