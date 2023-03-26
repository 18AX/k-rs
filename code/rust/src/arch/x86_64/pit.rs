use log::info;
use x86_64::structures::idt::InterruptStackFrame;

use crate::{arch::x86_64::pic, io::IOPort, timer::Timer};

use super::interrupt;

const CONTROLER_REGISTER: u16 = 0x43;
const INTERNAL_FREQ: u32 = 1193182;

// Read write LSB first and then MSB
const REGISTER_BOTH: u8 = 0x30;

type Result<T> = core::result::Result<T, PitError>;

#[derive(Debug)]
pub enum PitError {
    InvalidFrequency,
}

pub enum CounterRegister {
    First,
    Second,
    Third,
}

impl CounterRegister {
    pub fn id(&self) -> u8 {
        match self {
            CounterRegister::First => 0x0,
            CounterRegister::Second => 0x1,
            CounterRegister::Third => 0x2,
        }
    }

    pub fn port(&self) -> u16 {
        match self {
            CounterRegister::First => 0x40,
            CounterRegister::Second => 0x41,
            CounterRegister::Third => 0x42,
        }
    }
}

pub enum Mode {
    InterruptOnCount,
    HardwareOneShot,
    RateGenerator,
    SquareGenerator,
    SoftwareStrobe,
    HardwareStrobe,
}

impl From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::InterruptOnCount => 0x0,
            Mode::HardwareOneShot => 0x1,
            Mode::RateGenerator => 0x2,
            Mode::SquareGenerator => 0x3,
            Mode::SoftwareStrobe => 0x4,
            Mode::HardwareStrobe => 0x5,
        }
    }
}

pub static mut PIT_TIMER: Pit = Pit::new();

extern "x86-interrupt" fn pit_handler(_: InterruptStackFrame) {
    unsafe {
        PIT_TIMER.increment();
    }

    pic::eoi_master();
}

pub fn init_counter(freq: u32, mode: Mode, register: CounterRegister) -> Result<()> {
    let controller_port: IOPort<u8> = IOPort::new(CONTROLER_REGISTER);

    let mode: u8 = mode.into();
    let parameter: u8 = (mode << 1) | REGISTER_BOTH | (register.id() << 6);

    controller_port.write(parameter);

    let divider: u32 = INTERNAL_FREQ / freq;

    let divider: u16 = match divider.try_into() {
        Ok(d) => d,
        Err(_) => return Err(PitError::InvalidFrequency),
    };

    let reg_port: IOPort<u8> = IOPort::new(register.port());

    let divider_lsb: u8 = (divider & 0xFF) as u8;
    let divider_msb: u8 = (divider >> 8) as u8;

    reg_port.write(divider_lsb);
    reg_port.write(divider_msb);

    interrupt::register_handler(pic::MASTER_OFFSET, pit_handler);

    Ok(())
}

pub struct Pit {
    ticks: u64,
}

impl Pit {
    pub const fn new() -> Self {
        Pit { ticks: 0 }
    }

    fn increment(&mut self) {
        self.ticks += 1;
    }
}

impl Timer for Pit {
    fn get_ticks(&self) -> u64 {
        self.ticks
    }

    fn get_freq(&self) -> u32 {
        let port: IOPort<u8> = IOPort::new(CounterRegister::First.port());

        let divider_lsb: u32 = port.read().into();
        let divider_msb: u32 = port.read().into();

        let divider: u32 = divider_lsb | (divider_msb << 8);

        INTERNAL_FREQ / divider
    }
}
