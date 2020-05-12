use std::{thread, time};

mod player;
use player::Player;

mod battle;
use crate::battle::*;

mod framework;
use crate::framework::Event;

static TIMER: time::Duration = time::Duration::from_secs(1);
pub fn gcd() {
    thread::sleep(TIMER);
}

fn main() {
    let mut p = Player::new();

    let mut b = Battle::new(&mut p);

    loop {
        match b.turn() {
            Some(txt) => println!("{}", txt),
            None => {
                println!("Battle finished!");
                b = Battle::new(&mut p);
            }
        }

        gcd();
    }
}
