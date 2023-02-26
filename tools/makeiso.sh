#!/bin/sh
set -e

BUILD_DIR=build

# Delete old files
rm -rf $BUILD_DIR/iso/

mkdir -p $BUILD_DIR/iso/limine

cd $BUILD_DIR
cmake ../code/
make

cd ..

cp tools/limine.cfg $BUILD_DIR/iso/limine/limine.cfg
cp limine/limine.sys $BUILD_DIR/iso/limine/limine.sys
cp limine/limine-cd.bin $BUILD_DIR/iso/limine/limine-cd.bin
cp limine/limine-cd-efi.bin $BUILD_DIR/iso/limine/limine-cd-efi.bin
cp $BUILD_DIR/krs.elf $BUILD_DIR/iso/krs.elf


xorriso -as mkisofs -b limine/limine-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine/limine-cd-efi.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        $BUILD_DIR/iso -o $BUILD_DIR/krs.iso

./limine/limine-deploy $BUILD_DIR/krs.iso