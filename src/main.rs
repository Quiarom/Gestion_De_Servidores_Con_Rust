use std::net::{TcpStream, TcpListener};

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("¡Una visita!");
            } Err(e) => {
                println!("Conexión fallida, F");
            }
        }
    }

}
