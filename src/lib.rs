#![no_std]
use core::panic::PanicInfo;
use serial::Serial;

mod io;
mod serial;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn k_main() {
    let serial: Serial = match Serial::new(serial::COM1, 115200) {
        Ok(s) => s,
        Err(_) => loop {},
    };

    serial.write_byte('h' as u8);
    serial.write_byte('e' as u8);
    serial.write_byte('l' as u8);
    serial.write_byte('l' as u8);
    serial.write_byte('o' as u8);
    serial.write_byte('\r' as u8);
    serial.write_byte('\n' as u8);

    loop {}
}
