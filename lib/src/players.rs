use crate::card_set::CardSet;
use crate::users::User;
use anyhow::Result;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

pub struct Player {
    user: User,
    handcard: CardSet,
    role: Roles,
    stage: Stage,
}
impl Player {
    pub fn new(user: User) -> Self {
        Self {
            user,
            handcard: CardSet::new(),
            role: Roles::TheThird,
            stage: Stage::WaitingStage,
        }
    }
    pub fn set_role(&mut self, role: Roles) {
        self.role = role;
    }
    pub fn draw_cards(&mut self, cards: &mut CardSet) -> Result<()> {
        self.handcard.append(cards.draw_cards(17)?);
        Ok(())
    }
    pub fn remove_set(&mut self, cards: &mut CardSet) -> Result<()> {
        self.handcard.remove_set(cards)?;
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum Roles {
    Landlord,
    Farmer,
    TheThird,
}

enum Stage {
    WaitingStage,
    ClaimStage,
    CycleStage,
}
