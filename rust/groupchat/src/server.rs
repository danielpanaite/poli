use std::io::prelude::*;
//use std::io::{BufRead, BufReader, BufWriter};
use std::io::Write;
use std::net::{TcpStream, TcpListener, Shutdown};

pub fn start_server() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { println!("Accept failed: {:?}", e); }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            for i in 0..size{
                print!("{}", data[i] as char);
            }
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

// fn handle_connection(stream: TcpStream) -> std::io::Result<()> {

//     let mut br: BufReader<&TcpStream> = BufReader::new(&stream);
//     let mut bw: BufWriter<&TcpStream> = BufWriter::new(&stream);
//     let mut s = String::new();
//     loop{
//         let size = br.read_line(&mut s)?;
//         if size == 0 { return Ok(()); }

//         s = "Welcome, ".to_string() + &s + "!";

//         bw.write(s.as_bytes())?;
//         bw.flush().unwrap();
//         s.clear();
//     }
    
// }