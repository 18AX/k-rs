pub trait Timer {
    fn get_tick(&self) -> u64;
}
