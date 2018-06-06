use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Read, Write};

fn handle_client(stream: TcpStream) {
    println!("Connected !!");
    let mut stream = BufReader::new(stream);
    let mut buf = String::new();

    stream.read_line(&mut buf);
    println!("{}", buf);
}


fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listen !!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(_) => { println!("Failed") }
        }
    }

    Ok(())
}
