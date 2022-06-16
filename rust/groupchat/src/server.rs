use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpStream, TcpListener};

pub fn start_server() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:34254")?;

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                handle_connection(stream).unwrap();
            }
            Err(e) => { println!("Accept failed: {:?}", e); }
        }
    }

    Ok(())
}

fn handle_connection(stream: TcpStream) -> std::io::Result<()> {

    let mut br: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut bw: BufWriter<&TcpStream> = BufWriter::new(&stream);
    let mut s = String::new();
    loop{
        let size = br.read_line(&mut s)?;
        if size == 0 { return Ok(()); }

        s = "Welcome, ".to_string() + &s + "!";

        bw.write(s.as_bytes())?;
        bw.flush().unwrap();
        s.clear();
    }
    
}