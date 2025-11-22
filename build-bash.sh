#!/usr/bin/env bash
set -euo pipefail

mkdir -p "build"

bash_repo="${bash_repo:-https://git.savannah.gnu.org/git/bash.git}"
bash_tag="${bash_tag:-bash-5.3}"

pushd "build"

if [ ! -d "bash" ]; then
    git clone --depth 1 --branch "$bash_tag" "$bash_repo" "bash"
fi

if [ ! -f "bash/bash" ]; then
    pushd "bash"
    ./configure --without-bash-malloc --enable-static-link --host=aarch64-linux-gnu CC=aarch64-linux-gnu-gcc
    make
    popd
fi
popd