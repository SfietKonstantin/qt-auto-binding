#!/usr/bin/env bash
set -euxo pipefail

rustup self update

if [[ "$TRAVIS_OS_NAME" == "linux" ]]
then
    sudo apt-get update
    sudo apt-get install libqt4-dev qt5-default
fi

if [[ "$TRAVIS_OS_NAME" == "osx" ]]
then
    brew install qt
    ls /usr/local/Cellar/qt/5.11.2_1/lib
fi

if [[ "$TRAVIS_OS_NAME" == "windows" ]]
then
    if [[ ! -d /c/Qt ]]
    then
        curl -OL http://download.qt.io/official_releases/qt/5.11/5.11.2/qt-opensource-windows-x86-5.11.2.exe
        ./qt-opensource-windows-x86-5.11.2.exe --script ci/install.qs
    fi
    ls /c/Qt/
    ls /c/Qt/dist/
fi
