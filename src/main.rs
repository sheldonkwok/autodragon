use rand::Rng;
use std::{thread, time};

static TIMER: time::Duration = time::Duration::from_secs(1);

trait CanFight {
    fn attack(&self) -> u64;
    fn defend(&mut self, damage: u64) -> u64;
}

struct Monster {
    health: u64,
    damage: u64,
}

impl Monster {
    pub fn new() -> Monster {
        let health = rand::thread_rng().gen_range(10, 100);
        Monster { health, damage: 10 }
    }
}

impl CanFight for Monster {
    fn attack(&self) -> u64 {
        return self.damage;
    }

    fn defend(&mut self, damage: u64) -> u64 {
        if damage > self.health {
            self.health = 0;
        } else {
            self.health = self.health - damage;
        }

        return self.health;
    }
}

struct Player {
    health: u64,
    strength: u64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            strength: 10,
            health: 100,
        }
    }
}

impl CanFight for Player {
    fn attack(&self) -> u64 {
        return self.strength;
    }

    fn defend(&mut self, damage: u64) -> u64 {
        if damage >= self.health {
            self.health = 1 // You can't die!
        } else {
            self.health = self.health - damage;
        }

        return self.health;
    }
}

fn main() {
    let mut p = Player::new();

    loop {
        println!("New encounter!");
        let mut m = Monster::new();

        println!("Monster health: {}", m.health);
        while m.defend(p.attack()) > 0 {
            p.defend(m.attack());

            println!("Player health: {}", p.health);
            println!("Monster health: {}", m.health);
            thread::sleep(TIMER);
        }
    }
}
