pub trait Timer {
    fn get_ticks(&self) -> u64;
    fn get_freq(&self) -> u32;
}
