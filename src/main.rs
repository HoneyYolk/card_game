#![allow(unused)]
use card_game::card_set::Card;
use card_game::card_set::CardSet;

fn main() {
    let mut card_set = CardSet::gen_set();
    println!("{}", card_set);
}
