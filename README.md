# Rusty OS

Build with:

`cargo bootimage`

And then run it with (using QEMU):

`cargo run`

To convert it to a Virtualbox Machine, run:

`dd if=target/x86_64-rusty-os/debug/bootimage-rusty-os.bin of=rusty-os.bin bs=1M conv=sync` and then
`VBoxManage convertfromraw rusty-os.bin rusty-os.vdi --format VDI`

Setup:

`rustup override add nightly`

`cargo install bootimage`

`rustup component add llvm-tools-preview`

`rustup component add rust-src --toolchain {current toolchain}`

Cool trick (only able to do on Windows):

If you flash it with BalenaEtcher (idk if it works with other flash programs), you can also use it as a storage device.
Here's what you gotta do:
Open Disk Management, and select the disk you flashed the os on.
Then right click on Unallocated.
And click New Basic Volume.
Go through the process and you're done.
