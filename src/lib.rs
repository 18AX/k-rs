#![no_std]

use core::panic::PanicInfo;

use io::IOPort;

mod io;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn k_main() {
    let com1: IOPort<char> = IOPort::new(0x3F8);

    com1.write('h');
    com1.write('e');
    com1.write('l');
    com1.write('l');
    com1.write('o');
    com1.write('\r');
    com1.write('\n');

    loop {}
}
