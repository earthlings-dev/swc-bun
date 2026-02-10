#!/usr/bin/env bash
set -eu

# Usage: Invoke this script using full path from a next app.

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
echo "Script dir: $SCRIPT_DIR"
export NEXT_DEBUG_MINIFY=1

# Remove preovious builds
rm -rf .next

# Install latest canary
# bun add next@canary

# This is not useful data, as this is a script used only by me (@kdy1)
bunx next telemetry disable

bunx next build | bun "$SCRIPT_DIR/evaluate.js"
touch "$SCRIPT_DIR/../../tests/compress.rs"
