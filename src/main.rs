#![no_std] // don't link the Rust standard library
#![no_main] // disable the rust level entry points

use core::panic::PanicInfo;

// this function called on panic
#[panic_handler] // this is a lang item.
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// never change the _start name
#[unsafe(no_mangle)]
// entry point
pub extern "C" fn _start() -> ! {
    loop{}
}

