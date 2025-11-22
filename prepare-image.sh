#!/usr/bin/env bash
set -euo pipefail

base="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
export base

pushd "$base"

mkdir -p "build"

if [ ! -f "build/busybox-aarch64-linux-gnu" ]; then
    pushd "build"
    wget https://github.com/shutingrz/busybox-static-binaries-fat/raw/refs/heads/main/busybox-aarch64-linux-gnu
    chmod +x busybox-aarch64-linux-gnu
    popd
fi

./build-bash.sh
./mkrootfs-aarch64.sh