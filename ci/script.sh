#!/usr/bin/env bash
set -euxo pipefail

cargo build --verbose
cargo test --verbose

cd tests
cargo test --verbose