# k-rs

Implementation of the K project in Rust.

## How to build ?

Build limine binaries
```sh
cd limine && make
```

```sh
./tools/makeiso.sh
qemu-system-x86_64 -cdrom build/krs.iso -serial stdio
```