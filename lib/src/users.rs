use std::fmt;
use uuid::Uuid;

#[derive(Eq, Clone, Copy)]
pub struct User {
    name: &'static str,
    uuid: Uuid,
}
impl User {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4(),
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
