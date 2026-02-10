#!/usr/bin/env bash
set -eu

bun run build:dev
bun link

(cd swr && bun run build)