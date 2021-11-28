use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Message<T> {
    client_id: Uuid,
    msg: T,
}

impl<T> Message<T> {
    pub fn new(client_id: Uuid, msg: T) -> Self {
        Self { client_id, msg }
    }
}
