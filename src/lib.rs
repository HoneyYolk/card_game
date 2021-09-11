pub mod card_set;
pub mod desk;
pub mod game;
pub mod messager;
pub mod player;
pub mod rules;

#[cfg(test)]
mod test {
    use super::card_set::*;
    #[test]
    fn test_1() {
        let mut handcard = CardSet::gen_set();
        let mut set = CardSet::new();
        for _ in 0..4 {
            set.push(Card::new(0, 4));
        }
        assert!(!handcard.remove_set(&set).is_err());
    }
}
