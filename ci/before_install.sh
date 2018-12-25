#!/usr/bin/env bash
set -euxo pipefail

rustup self update

if [[ "$TRAVIS_OS_NAME" == "linux" ]]
then
    sudo apt-get update
    sudo apt-get install qt5-default qtdeclarative5-dev
fi

if [[ "$TRAVIS_OS_NAME" == "osx" ]]
then
    brew install qt
fi
