#!/bin/bash
set -euo pipefail

# This wraps everything to avoid truncated script issues.
__wrap__() {

    # Download or use cached binary
    BINARY_PATH=$(./download.sh)

    # Execute the binary with all passed arguments
    if [ -x "$BINARY_PATH" ]; then
        "$BINARY_PATH" "$@"
    else
        echo "Failed to execute the binary."
        exit 1
    fi

}

__wrap__
