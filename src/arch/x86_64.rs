use log::info;

pub mod interrupt;
pub mod pic;
pub mod pit;

pub fn init() {
    interrupt::init();
    pic::init();

    info!(target:"x86_64_init", "Using PIC for IRQs");
}
