use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use rand::{seq::SliceRandom, thread_rng};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = CardSet::gen_set();
        println!("{}", a.cards.contains(&Card::new(0, 4)));
    }
}

#[derive(Clone)]
pub struct CardSet {
    cards: Vec<Card>,
}
impl CardSet {
    pub fn new(cards: Vec<Card>) -> CardSet {
        CardSet { cards }
    }
    pub fn iter(&'_ mut self) -> core::slice::Iter<Card> {
        self.cards.iter()
    }
    pub fn gen_set() -> CardSet {
        let mut set = Vec::new();
        let mut rng = thread_rng();
        for i in 0..52 {
            set.push(Card::new(i / 4, i % 4));
        }
        set.push(Card::new(13, 4));
        set.push(Card::new(14, 4));
        set.shuffle(&mut rng);
        CardSet { cards: set }
    }

    pub fn sort(&mut self) {
        self.cards.sort();
    }
    pub fn draw_cards(&mut self, amount: usize) -> Result<CardSet, &'static str> {
        if amount > self.cards.len() {
            return Err("Too many cards");
        }
        let mut cards = Vec::new();
        for _ in 0..amount {
            if let Some(card) = self.cards.pop() {
                cards.push(card);
            }
        }
        cards.sort();
        Ok(CardSet { cards })
    }
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn push_set(&mut self, set: CardSet) {
        let mut set = set;
        loop {
            if let Some(card) = set.cards.pop() {
                self.cards.push(card);
            }
        }
    }
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    pub fn find(&self, card: &Card) -> Option<usize> {
        for (i, c) in self.cards.iter().enumerate() {
            if c == card {
                return Some(i);
            }
        }
        None
    }
    pub fn remove(&mut self, card: &Card) -> Result<(), &'static str> {
        match self.find(card) {
            Some(i) => {
                self.cards.remove(i);
                Ok(())
            }
            None => Err("No such card"),
        }
    }
    pub fn remove_set(&mut self, set: &CardSet) -> Result<(), &'static str> {
        let mut set = set.clone();
        let mut cards = self.clone();
        for c in set.iter() {
            cards.remove(c)?;
        }
        self.cards = cards.cards;
        Ok(())
    }
}

impl fmt::Display for CardSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for c in self.cards.iter() {
            str.push_str(c.to_string());
        }
        write!(f, "{}", str)
    }
}

#[derive(Eq, Clone, Debug)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}
impl Card {
    pub fn new(rank: u32, suit: u32) -> Card {
        Card {
            rank: Rank::from_num(rank),
            suit: Suit::from_num(suit),
        }
    }
    pub fn to_string(&self) -> &str {
        match self.rank {
            Rank::Card0 => "牛大逼0号牌",
            Rank::Card3 => "3",
            Rank::Card4 => "4",
            Rank::Card5 => "5",
            Rank::Card6 => "6",
            Rank::Card7 => "7",
            Rank::Card8 => "8",
            Rank::Card9 => "9",
            Rank::Card10 => "10",
            Rank::CardJ => "J",
            Rank::CardQ => "Q",
            Rank::CardK => "K",
            Rank::CardA => "A",
            Rank::Card2 => "2",
            Rank::CardJoker => "鬼",
            Rank::CardKing => "王",
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => self.suit.cmp(&other.suit),
            _ => self.rank.cmp(&other.rank),
        }
    }
}
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.suit == Suit::Suit0 || other.suit == Suit::Suit0 {
            self.rank == other.rank
        } else {
            self.rank == other.rank && self.suit == other.suit
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
enum Rank {
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    Card10,
    CardJ,
    CardQ,
    CardK,
    CardA,
    Card2,
    CardJoker,
    CardKing,
    Card0,
}
impl Rank {
    fn from_num(num: u32) -> Rank {
        if let Some(rank) = Rank::from_u32(num) {
            rank
        } else {
            Rank::Card0
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
enum Suit {
    Heart,   //红桃♥
    Diamond, //方片♦
    Club,    //梅花♣
    Spade,   //黑桃♠
    Suit0,   //双王花色
}
impl Suit {
    fn from_num(num: u32) -> Suit {
        if let Some(suit) = Suit::from_u32(num) {
            suit
        } else {
            Suit::Suit0
        }
    }
}
