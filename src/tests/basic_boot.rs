#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(jarvos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use jarvos::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    jarvos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println("Hello world!");
}
