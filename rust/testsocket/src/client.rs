use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};

pub fn send_to_server(address: String, message: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(address)?;

    stream.write(message.as_bytes());
    stream.shutdown(Shutdown::Write);

    let mut reply = String::new();
    stream.read_to_string(&mut reply);
    println!("Server reply: {}", reply);

    Ok(reply)
}