use anyhow::{anyhow, Result};

pub enum Event {
    JoinGame,
    GameStart,
}
impl Event {
    pub fn event_convertor(input: String) -> Result<Event> {
        match input.as_str() {
            "join" => Ok(Event::JoinGame),
            "start" => Ok(Event::GameStart),
            _ => Err(anyhow!("Invalid event")),
        }
    }
}

pub fn process_event(event: Event) {}
