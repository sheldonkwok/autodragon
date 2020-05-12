use rand::Rng;

use crate::framework::CanFight;

pub struct Monster {
    health: u64,
    damage: u64,
}

impl Monster {
    pub fn new() -> Self {
        let health = rand::thread_rng().gen_range(10, 100);
        Monster { health, damage: 10 }
    }
}

impl CanFight for Monster {
    fn health(&self) -> u64 {
        return self.health;
    }

    fn attack(&self) -> u64 {
        return self.damage;
    }

    fn defend(&mut self, damage: u64) -> (u64, bool) {
        if damage > self.health {
            self.health = 0;
            return (damage - self.health, true);
        }

        self.health = self.health - damage;
        return (damage, false);
    }
}
