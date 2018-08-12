#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}
