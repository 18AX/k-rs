#![no_std]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![feature(panic_info_message)]

extern crate alloc;

use alloc::{boxed::Box, string::String};
use lazy_static::lazy_static;
use log::{info, Level};
use memory::kernel_heap;
use serial::Serial;

use crate::{arch::x86_64::interrupt, panic::die};

mod arch;
mod io;
mod memory;
mod panic;
mod serial;

lazy_static! {
    static ref SERIAL: Serial = match Serial::new(serial::COM1, 115200) {
        Ok(s) => s,
        Err(_) => panic!("Failed to initialize serial"),
    };
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    kernel_heap::init();

    if log::set_logger(&*SERIAL).is_err() {
        SERIAL.write_string(&String::from("Error"));
    }

    log::set_max_level(Level::Debug.to_level_filter());

    info!(target:"k_main", "Kernel starting");

    interrupt::init();

    let b = Box::new(42);

    drop(b);

    unsafe {
        core::arch::asm!("int 0x0");
    };

    die();
}
