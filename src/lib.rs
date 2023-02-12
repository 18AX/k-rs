#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use core::panic::PanicInfo;
use memory::kernel_heap;
use serial::Serial;

mod io;
mod memory;
mod serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn k_main() {
    kernel_heap::init();

    let serial: Serial = match Serial::new(serial::COM1, 115200) {
        Ok(s) => s,
        Err(_) => loop {},
    };

    let b = Box::new(42);

    serial.write_byte('h' as u8);
    serial.write_byte('e' as u8);
    serial.write_byte('l' as u8);
    serial.write_byte('l' as u8);
    serial.write_byte('o' as u8);
    serial.write_byte('\r' as u8);
    serial.write_byte('\n' as u8);

    drop(b);

    loop {}
}
