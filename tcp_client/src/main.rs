use std::{net::TcpStream, io::Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("hello world!".as_bytes()).unwrap();
}
