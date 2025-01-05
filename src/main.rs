use bincode::Options;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpListener;

#[derive(Deserialize)]
struct Request {
    _message_size: u32,
    _request_api_key: u16,
    _request_api_version: u16,
    correlation_id: u32,
}

#[derive(Serialize)]
struct Response {
    message_size: u32,
    correlation_id: u32,
}

fn main() {
    let options = bincode::options().with_big_endian().with_fixint_encoding();
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                const REQUEST_SIZE: usize = std::mem::size_of::<Request>();
                let mut request = [0u8; REQUEST_SIZE];
                assert_eq!(stream.read(&mut request).unwrap(), REQUEST_SIZE);
                let request = options.deserialize::<Request>(&request).unwrap();
                let resp = Response {
                    message_size: 0,
                    correlation_id: request.correlation_id,
                };
                let msg = options.serialize(&resp).unwrap();
                stream.write_all(&msg).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
