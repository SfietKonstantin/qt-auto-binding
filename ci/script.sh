#!/usr/bin/env bash
export QT_QPA_PLATFORM=offscreen
set -euxo pipefail

cargo build --verbose
cargo test --verbose

cd tests

pushd base
cargo test --verbose
popd

pushd gui
cargo test --verbose
popd

pushd widgets
cargo test --verbose
popd
