Build with:

`cargo bootimage -Z build-std -Z build-std-features`

And then run it with (using QEMU):

`qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-os.bin`

Setup:

`rustup override add nightly`

`cargo install bootimage`

`rustup component add llvm-tools-preview`