#!/bin/bash
# The purpose of this script is to run the tests without displaying any of the successful build messages
clear
cargo test \
| grep -v "Building bootloader" \
| grep -v "Compiling" \
| grep -v "^Running: \`qemu-system-x86_64"