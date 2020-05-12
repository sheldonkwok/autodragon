use crate::player::Player;

use crate::battle::monster::Monster;
use crate::framework::{CanFight, Event};

pub struct Battle<'a> {
    monster: Monster,
    player: &'a mut Player,
    turn: u32,
    finished: bool,
}

impl<'a> Battle<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        Battle {
            monster: Monster::new(),
            player,
            turn: 1,
            finished: false,
        }
    }
}

impl<'a> Event for Battle<'a> {
    fn turn(&mut self) -> Option<String> {
        if self.finished {
            return None;
        }

        if self.turn % 2 == 1 {
            let (monster_dmg_taken, slain) = self.monster.defend(self.player.attack());

            if slain {
                self.player.gain_exp(1);
                self.finished = true;
            }

            return Some(format!(
                "[Player: {}] | [Monster: {}]: Player attacks for {}",
                self.player.health(),
                self.monster.health(),
                monster_dmg_taken
            ));
        }

        let (player_dmg_taken, _) = self.player.defend(self.monster.attack());

        return Some(format!(
            "[Player: {}] | [Monster: {}]: Monster attacks for {}",
            self.player.health(),
            self.monster.health(),
            player_dmg_taken
        ));
    }
}
