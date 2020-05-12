use std::{thread, time};

mod framework;

mod player;
use player::Player;

mod battle;
use crate::battle::battle::*;

static TIMER: time::Duration = time::Duration::from_secs(1);
pub fn gcd() {
    thread::sleep(TIMER);
}

fn main() {
    let mut p = Player::new();

    let mut b = Battle::new(&mut p);

    loop {
        match b.next() {
            Some(txt) => println!("{}", txt),
            None => {
                println!("Battle finished!");
                b = Battle::new(&mut p);
            }
        }

        gcd();
    }
}
