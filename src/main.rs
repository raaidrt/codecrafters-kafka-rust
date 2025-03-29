#![allow(unused_imports)]
mod request;
mod response;
use deku::{DekuContainerRead, DekuContainerWrite};
use response::Response;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

use request::Request;

fn main() -> Result<(), Box<dyn Error>> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf: [u8; 96] = [0; 96];
                let n = stream.read(&mut buf)?;
                println!("read {n} bytes");
                let ((_, _), request) = Request::from_bytes((&buf, 0))?;

                let response = Response {
                    message_size: 0,
                    correlation_id: request.correlation_id,
                };
                let buf = response.to_bytes()?;
                stream.write(&buf)?;
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
