use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};

pub fn send_to_server(address: String, message: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(address)?;

    stream.write(message.as_bytes()).expect("Response send failed");
    stream.shutdown(Shutdown::Write).unwrap();

    let mut reply = String::new();
    stream.read_to_string(&mut reply).expect("Failed reading server response");
    println!("Server reply: {}", reply);

    Ok(reply)
}