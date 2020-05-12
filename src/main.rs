mod monster;
use monster::Monster;

mod player;
use player::Player;

mod framework;
use framework::{gcd, CanFight};

fn battle(p: &mut Player) {
    println!("New encounter!");
    let mut m = Monster::new();

    loop {
        let (monster_dmg_taken, slain) = m.defend(p.attack());
        println!(
            "[Player: {}] | [Monster: {}]: Player attacks for {}     ",
            p.health(),
            m.health(),
            monster_dmg_taken
        );

        if slain {
            break;
        }

        gcd();
        let (player_dmg_taken, _) = p.defend(m.attack());

        println!(
            "[Player: {}] | [Monster: {}]: Monster attacks for {}    ",
            p.health(),
            m.health(),
            player_dmg_taken
        );

        gcd();
    }

    p.gain_exp(1);
}

fn main() {
    let mut p = Player::new();

    loop {
        battle(&mut p);
        gcd();
    }
}
