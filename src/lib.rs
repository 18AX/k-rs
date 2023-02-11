#![no_std]
use core::arch::asm;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn k_main() {
    let port: u16 = 0x3F8;
    let value: u8 = 0x48;

    loop {
        unsafe {     asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
         ); }
    }
}
