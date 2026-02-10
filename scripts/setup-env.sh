#!/usr/bin/env bash
set -eu

NODE_PLATFORM_NAME=$(bun -e "console.log(require('os').platform())")


(cd scripts/npm/core-$NODE_PLATFORM_NAME && bun link)
bun link @swc/core-$NODE_PLATFORM_NAME
