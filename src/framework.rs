use std::{thread, time};

pub trait CanFight {
    fn health(&self) -> u64;
    fn attack(&self) -> u64;
    fn defend(&mut self, damage: u64) -> (u64, bool); // damage taken, slain
}

static TIMER: time::Duration = time::Duration::from_secs(1);
pub fn gcd() {
    thread::sleep(TIMER);
}
