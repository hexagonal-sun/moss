#!/usr/bin/env bash
set -euo pipefail

img="$base/test.img"
mount="$base/build/mount"

if [ -f "$img" ]; then
    echo "initrd file $img already exists.  Skipping."
    exit 0
fi

mkdir -p "$mount"

dd if=/dev/zero of="$img" bs=1M count=128
mkfs.vfat -F 32 "$img"

if ! mountpoint -q "$mount"; then
    fusefat -o rw+ "$img" "$mount"
fi

mkdir -p "$mount/bin"
mkdir -p "$mount/dev"

cp build/bash/bash "$mount/bin"

# busybox -- I couldn't get this to build.  I ended up restoring to a third-party static binary which isn't ideal but it get's things running.
cp build/busybox-aarch64-linux-gnu "$mount/bin"

if mountpoint -q "$mount"; then
    umount "$mount"
fi

