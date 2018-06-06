use std::net::{TcpListener, TcpStream};
use std::io;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    Ok(())
}
