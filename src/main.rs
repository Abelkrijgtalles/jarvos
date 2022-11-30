#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    println!("Done with {} tests", tests.len());
}

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(
        "Something bad happend:\n{}\nYou might need to restart your pc in order to fix the issue.",
        info
    );

    loop {}
}
