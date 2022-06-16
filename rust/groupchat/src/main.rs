mod client; 
mod server;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args[1] == "Server" {
        let _startserv = server::start_server();
    }else{
        let _reply = client::send_to_server("127.0.0.1:34254".to_string(), &args[1]);
    }

}
