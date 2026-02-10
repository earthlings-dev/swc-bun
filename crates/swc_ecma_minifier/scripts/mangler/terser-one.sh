#!/usr/bin/env bash
#
# Generates reference output for the mangler, using terser
# 
set -eu

output="${1/input/output.mangleOnly}"
bunx terser --mangle --toplevel --output "$output" $1
bunx prettier --write "$output"