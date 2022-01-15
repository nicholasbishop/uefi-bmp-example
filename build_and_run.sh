#!/bin/sh

set -eux

cargo +nightly build --target x86_64-unknown-uefi && uefi-run --bios /usr/share/edk2/ovmf/OVMF_CODE.fd target/x86_64-unknown-uefi/debug/uefi_bmp.efi 
