use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) {
    let mut reader = BufReader::new(&mut *stream);
    loop {
        let mut buf = vec![];
        reader.read_until(b'}', &mut buf).unwrap();
        let s = String::from_utf8(buf);
        println!("read s: {}", s.unwrap());
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:10888")?;
    for stream in listener.incoming() {
        std::thread::spawn(|| handle_client(&mut stream.unwrap()));
    }
    Ok(())
}
