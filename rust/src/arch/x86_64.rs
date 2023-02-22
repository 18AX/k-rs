use log::info;

use crate::timer::Timer;

pub mod interrupt;
pub mod pic;
pub mod pit;

pub fn init() {
    interrupt::init();
    pic::init();

    info!(target:"x86_64_init", "Using PIC for IRQs");

    if let Err(e) = pit::init_counter(100, pit::Mode::RateGenerator, pit::CounterRegister::First) {
        panic!("pit: {:?}", e);
    }

    unsafe {
        info!(target:"x86_64_init", "PIT frequency {}", pit::PIT_TIMER.get_freq());
    }

    pic::set_irq_state(pic::Irq::Pit, pic::IrqState::Unmask);

    interrupt::enable_irq(true);
}
