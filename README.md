Build with:

`cargo bootimage -Z build-std -Z build-std-features`

And then run it with (using QEMU):

`qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-os.bin`

Setup:

`rustup override add nightly`

`cargo install bootimage`

`rustup component add llvm-tools-preview`

Cool trick (only able to do on Windows):

If you flash it with BalenaEtcher (idk if it works with other flash programs), you can also use it as an storage device.
Here's what you gotta do:
Open Disk Management, and select the disk you flashed the os on.
Then right click on Unallocated.
And click New Basic Volume.
Go through the process and you're done.