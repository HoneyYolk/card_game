#![allow(unused)]
use lib::card_set::CardSet;
use lib::event_processor;
use lib::game::Game;
use lib::players::Player;

fn main() {
    loop {
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        event_processor::Event::event_convertor(command);
    }
}
