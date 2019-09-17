#!/usr/bin/env bash
export QT_QPA_PLATFORM=offscreen
set -euxo pipefail

cargo build --verbose
cargo test --verbose

cd tests
cargo test --verbose