use std::{net::TcpListener, io::Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];

        let size = stream.read(&mut buffer).unwrap();
        println!("recived: {:?}", String::from_utf8((&buffer[..size-1]).to_vec()).unwrap() );
    }
}
