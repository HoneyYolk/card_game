use std::convert::TryInto;

use crate::card_set::CardSet;
use crate::players::Player;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

pub struct Game {
    players: Vec<Player>,
    active_player: ActivePlayer,
    deadwood: Vec<(Player, CardSet)>,
    landcard: CardSet,
}





#[derive(Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
enum ActivePlayer {
    One,
    Two,
    Three,
    NoPlayer,
}
impl ActivePlayer {
    pub fn from_num(num: usize) -> Self {
        match ActivePlayer::from_u32(num.try_into().unwrap()) {
            Some(val) => val,
            None => ActivePlayer::NoPlayer,
        }
    }
}
impl Iterator for ActivePlayer {
    type Item = ActivePlayer;
    fn next(&mut self) -> Option<Self::Item> {
        use ActivePlayer::*;
        match self {
            One => Some(Two),
            Two => Some(Three),
            Three => Some(One),
            NoPlayer => None,
        }
    }
}
