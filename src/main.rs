#![no_std] // don't link the Rust standard library
#![no_main] // disable the rust level entry points

use core::panic::PanicInfo;

// this function called on panic
#[panic_handler] // this is a lang item.
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// Global variable (fat pointer) always 16 bytes. 
static HELLO: &[u8] = b"Hello world! from kernel!" ;


// never change the _start name 
// mangle is an unsafe feature.
#[unsafe(no_mangle)]
// entry point
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;  // raw pointer to VGABuffer
    for(i, &byte ) in HELLO.iter().enumerate(){ // HELLO = [(0,72),(1,101),(2,108),(3,108),(4,111)]
        unsafe { // unsafe because we are writing to memory, rust won't allow that without unsafe block.
            // multiply on 2 because each char needs 2 bytes : 
                // 1. byte for char (ASCII)
                // 2. byte for color (Color Attribute)

                
            *vga_buffer.offset(i as isize * 2) = byte; 
            *vga_buffer.offset(i as isize * 2 + 1) = 0x5; // 0x5 means Magenta
        }
        // after each loop, we calculate the offset based on the first address. it's not by increment.
    }
    loop{}

}

