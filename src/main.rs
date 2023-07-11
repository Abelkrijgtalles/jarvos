#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "This is a kind of native println! but also not! 420/69: {}",
        420.0 / 69.0
    );

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}. Rebooting your pc may help.", info);
    loop {}
}
