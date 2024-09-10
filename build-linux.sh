#!/bin/bash

set -e

mkdir -p build/
pushd build
cmake -DCMAKE_BUILD_TYPE=Release .. -DINSTALL_MANPAGES=OFF -DWITH_OGG=OFF
cmake --build .
popd
