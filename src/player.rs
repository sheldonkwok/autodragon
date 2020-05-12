use crate::framework::CanFight;

pub struct Player {
    health: u64,
    strength: u64,

    exp: u64,
    level: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            strength: 10,
            health: 100,
            exp: 0,
            level: 0,
        }
    }

    pub fn gain_exp(&mut self, exp: u64) {
        if exp == 0 {
            return;
        }

        self.exp = self.exp + exp;
        self.level = 1 + self.exp / 10
    }
}

impl CanFight for Player {
    fn health(&self) -> u64 {
        return self.health;
    }

    fn attack(&self) -> u64 {
        return self.strength;
    }

    fn defend(&mut self, damage: u64) -> (u64, bool) {
        if damage >= self.health {
            self.health = 1; // You can't die!
            return (damage - self.health, false);
        }

        self.health = self.health - damage;
        return (damage, false);
    }
}
