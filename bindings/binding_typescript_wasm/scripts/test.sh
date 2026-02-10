#!/usr/bin/env bash

set -eu

./scripts/build.sh
bun test $@

./scripts/build.sh --features nightly
bun test $@
