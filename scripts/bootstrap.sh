#!/bin/bash

# Define the base URL for the GitHub releases
REPO_URL="https://github.com/chainyo/py-init-cleaner/releases/latest/download"

# Determine platform details
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
if [[ "$ARCH" == "x86_64" ]]; then
    ARCH="x86_64"
elif [[ "$ARCH" == "aarch64" ]]; then
    ARCH="ARM64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

# Construct the binary name and URL
BINARY_NAME="py-init-cleaner-${OS}-${ARCH}"
DOWNLOAD_URL="$REPO_URL/$BINARY_NAME"

# Download the binary
curl -L $DOWNLOAD_URL -o $BINARY_NAME

# Make the binary executable
chmod +x $BINARY_NAME

# Execute the binary
./$BINARY_NAME "$@"
