#!/bin/bash
set -euo pipefail

VERSION=${PIC_VERSION:-latest}
REPO="chainyo/py-init-cleaner"
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $PLATFORM in
    Darwin) PLATFORM="macos" ;;
    Linux) PLATFORM="linux" ;;
esac

case $ARCH in
    armv8*|arm64*|aarch64*) ARCH="aarch64" ;;
    i686*) ARCH="x86" ;;
    *) ARCH="x86_64" ;;
esac

BINARY="py-init-cleaner-${ARCH}-${PLATFORM}"
BINARY_PATH="$HOME/.cache/pre-commit/$BINARY"

# Check if binary is already downloaded
if [[ ! -f "$BINARY_PATH" ]]; then
    echo "Downloading $BINARY..."
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}.gz"
    curl -SL --progress-bar "$DOWNLOAD_URL" -o "${BINARY_PATH}.gz"
    gunzip "${BINARY_PATH}.gz"
    chmod +x "$BINARY_PATH"
else
    echo "Using cached binary."
fi

echo "$BINARY_PATH"
