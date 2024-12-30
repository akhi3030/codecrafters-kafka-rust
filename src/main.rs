use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let buf: [u8; 8] = [0u8, 0, 0, 0, 0, 0, 0, 7];
                stream.write_all(&buf).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
