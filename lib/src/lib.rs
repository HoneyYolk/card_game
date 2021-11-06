pub mod card_set;
pub mod event_processor;
pub mod game;
pub mod message;
pub mod players;
pub mod rules;
pub mod users;

#[cfg(test)]
mod test {
    use super::card_set::*;
    #[test]
    fn test_1() {
        let mut handcard = CardSet::gen_deck();
        let mut set = CardSet::new();
        for _ in 0..4 {
            set.push(Card::new(0, 4));
        }
        assert!(handcard.remove_set(&set).is_ok());
    }
}
