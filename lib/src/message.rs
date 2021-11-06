use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    user_id: Uuid,
    msg: String,
}

impl Message {
    pub fn new(user_id: Uuid, msg: String) -> Self {
        Self { user_id, msg }
    }
}
