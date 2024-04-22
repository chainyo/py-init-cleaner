#!/bin/bash
set -euo pipefail

VERSION=${PIC_VERSION:-latest}
REPO="chainyo/py-init-cleaner"
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [[ $PLATFORM == "darwin" ]]; then
  PLATFORM="macos"
fi

if [[ $ARCH == armv8* ]] || [[ $ARCH == arm64* ]] || [[ $ARCH == aarch64* ]]; then
  ARCH="aarch64"
elif [[ $ARCH == i686* ]]; then
  ARCH="x86"
fi

BINARY="py-init-cleaner-${ARCH}-${PLATFORM}"
BINARY_PATH="$HOME/.cache/pre-commit/$BINARY"

# Oddly enough GitHub has different URLs for latest vs specific version
if [[ $VERSION == "latest" ]]; then
  DOWNLOAD_URL=https://github.com/${REPO}/releases/latest/download/${BINARY}.gz
else
  DOWNLOAD_URL=https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}.gz
fi

# Check if binary is already downloaded
if [[ ! -f "$BINARY_PATH" ]]; then
    curl -SL --progress-bar "$DOWNLOAD_URL" -o "${BINARY_PATH}.gz"

    # Ensure the file is in gzip format
    file "${BINARY_PATH}.gz" # This will print the file type

    # Proceed only if the file is a gzip file
    if file "${BINARY_PATH}.gz" | grep -q 'gzip compressed data'; then
        gunzip "${BINARY_PATH}.gz"
        chmod +x "$BINARY_PATH"
    else
        exit 1
    fi
fi

echo "$BINARY_PATH"
