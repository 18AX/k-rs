use core::{arch::asm, panic::PanicInfo};

use log::error;

pub fn die() -> ! {
    loop {
        unsafe {
            asm!("pause");
        };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.message() {
        Some(m) => error!(target:"panic", "{:?}", m),
        None => error!(target:"panic", "Unknown reason"),
    }

    die();
}
