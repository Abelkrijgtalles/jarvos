#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jarvos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "This is a kind of native println! but also not! 420/69: {}",
        420.0 / 69.0
    );

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}. Rebooting your pc may help.", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jarvos::test_panic_handler(info)
}
