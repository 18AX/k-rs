#!/bin/sh
set -e

# Delete old files
rm -rf iso/

mkdir -p iso/limine

cp limine.cfg iso/limine/limine.cfg
cp limine/limine.sys iso/limine/limine.sys
cp limine/limine-cd.bin iso/limine/limine-cd.bin
cp limine/limine-cd-efi.bin iso/limine/limine-cd-efi.bin
cp build/krs.elf iso/krs.elf


xorriso -as mkisofs -b limine/limine-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine/limine-cd-efi.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        iso -o krs.iso

./limine/limine-deploy krs.iso