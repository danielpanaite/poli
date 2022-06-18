use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};

pub fn send_to_server(address: String, message: &str) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(address)?;
    let mut s = String::new();
    let mut reply = String::new();
    loop{
        
        if std::io::stdin().read_line(&mut s).is_ok(){
            println!("Readline ok");
        }
        stream.write(s.as_bytes()).expect("Response send failed");
        //stream.shutdown(Shutdown::Write).unwrap();
        
        stream.read_to_string(&mut reply).expect("Failed reading server response");
        println!("Server reply: {}", reply);
    }
    
    Ok(reply)
}