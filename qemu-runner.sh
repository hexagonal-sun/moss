#!/usr/bin/env bash
set -e

ELF="$1"
BIN="${ELF%.elf}.bin"

./prepare-image.sh

# Convert to binary format
aarch64-none-elf-objcopy -O binary "$ELF" "$BIN"
qemu-system-aarch64 -M virt,gic-version=3 -initrd test.img -cpu cortex-a72 -m 2G -smp 4 -nographic -s -kernel "$BIN"
