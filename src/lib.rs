#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;

use alloc::{boxed::Box, string::String};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use log::{error, info, Level};
use memory::kernel_heap;
use serial::Serial;

mod io;
mod memory;
mod serial;

lazy_static! {
    static ref SERIAL: Serial = match Serial::new(serial::COM1, 115200) {
        Ok(s) => s,
        Err(_) => panic!("Failed to initialize serial"),
    };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!(target:"panic", "{:?}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn k_main() {
    kernel_heap::init();

    if log::set_logger(&*SERIAL).is_err() {
        SERIAL.write_string(&String::from("Error"));
    }

    log::set_max_level(Level::Debug.to_level_filter());

    info!(target:"k_main", "Kernel starting");

    let b = Box::new(42);

    drop(b);

    loop {}
}
