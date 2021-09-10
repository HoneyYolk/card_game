#![allow(unused)]
use card_game::card_set::CardSet;

fn main() {
    let mut a = CardSet::gen_set();
    let b = &a;
    let mut a = CardSet::gen_set();
    println!("{}", b.to_string());
    println!("{}", a.to_string());
}
