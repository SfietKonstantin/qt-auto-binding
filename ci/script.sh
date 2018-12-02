#!/usr/bin/env bash
set -euxo pipefail

cargo build --verbose
cargo test --verbose