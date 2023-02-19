use crate::timer::Timer;

struct Pit {
    counter: u64,
}

impl Timer for Pit {
    fn get_tick(&self) -> u64 {
        self.counter
    }
}
