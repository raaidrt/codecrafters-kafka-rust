pub mod request;
pub mod response;
pub mod size;

use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;

use deku::{DekuContainerRead, DekuContainerWrite};
use request::Request;

pub fn run_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:9092")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf: [u8; 96] = [0; 96];
                let _n = stream.read(&mut buf)?;
                let ((_, _), request) = Request::from_bytes((&buf, 0))?;

                println!("{:?}", request);
                let response = request.handle();
                let buf = response.to_bytes()?;
                stream.write(&buf)?;
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
    Ok(())
}
