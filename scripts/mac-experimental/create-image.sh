#!/usr/bin/env bash
set -euo pipefail

base="$( cd "$( dirname "${BASH_SOURCE[0]}" )"/../.. && pwd )"
pushd "$base" &>/dev/null || exit 1

img="$base/moss.img"
mount="$base/build/mount"

mkdir -p "$mount"

dd if=/dev/zero of="$img" bs=1M count=128
mkfs.vfat -F 32 "$img"

if ! mount | grep -q "$mount"; then
    hdiutil attach -mountpoint "$mount" "$img"
fi

mkdir -p "$mount/bin"
mkdir -p "$mount/dev"

cp "$base/build/bin"/* "$mount/bin"

mounted=$(mount | grep "on $mount " | awk '{print $1}')
if [ -n "$mounted" ]; then
    hdiutil detach "$mounted"
fi
popd &>/dev/null || exit 1
