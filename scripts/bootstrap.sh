#!/bin/bash
set -euo pipefail

# This wraps everything to avoid truncated script issues.
__wrap__() {
    # Determine the directory this script resides in
    SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

    # Download or use cached binary
    BINARY_PATH=$("$SCRIPT_DIR/download.sh")
    echo "Using binary: $BINARY_PATH"

    # Execute the binary with all passed arguments
    "$BINARY_PATH" "$@"
}

__wrap__
