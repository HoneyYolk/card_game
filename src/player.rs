use std::io;
use uuid::Uuid;

#[derive(Eq, Clone, Copy)]
pub struct Player {
    name: &'static str,
    uuid: Uuid,
}
impl Player {
    pub fn new(name: &'static str) -> Result<Self, uuid::Error> {
        Ok(Self {
            name,
            uuid: Uuid::new_v4(),
        })
    }
    pub fn send_message(&self, msg: &str) {
        self.sender(msg);
    }
    pub fn recieve_command(&self) -> String {
        self.reciever()
    }
    fn sender(&self, msg: &str) {
        println!("{}: {}", self.name, msg);
    }
    fn reciever(&self) -> String {
        let mut msg = String::new();
        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        msg
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.uuid == other.uuid
    }
}
