pub trait CanFight {
    fn health(&self) -> u64;
    fn attack(&self) -> u64;
    fn defend(&mut self, damage: u64) -> (u64, bool); // damage taken, slain
}
