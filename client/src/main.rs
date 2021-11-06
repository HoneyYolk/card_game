use lib::message::Message;
use std::io::{self, Write};
use std::net::TcpStream;
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8888")?;
    let user_id = Uuid::new_v4();
    loop {
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        let msg = Message::new(user_id, text);
        stream.write_all(serde_json::to_string(&msg).unwrap().as_bytes())?;
    }
}
