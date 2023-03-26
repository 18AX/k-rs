pub struct DummyTimer;

impl Timer for DummyTimer {
    fn get_ticks(&self) -> u64 {
        0
    }

    fn get_freq(&self) -> u32 {
        0
    }
}

static DUMMY_TIMER: DummyTimer = DummyTimer;

static mut GLOBAL_TIMER: &dyn Timer = &DUMMY_TIMER;

pub fn set_default(timer: &'static dyn Timer) {
    unsafe {
        GLOBAL_TIMER = timer;
    }
}

pub fn get_ticks() -> u64 {
    unsafe { GLOBAL_TIMER.get_ticks() }
}

pub fn get_freq() -> u32 {
    unsafe { GLOBAL_TIMER.get_freq() }
}

pub trait Timer {
    fn get_ticks(&self) -> u64;
    fn get_freq(&self) -> u32;
}
