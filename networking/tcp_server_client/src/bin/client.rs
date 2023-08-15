use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("127.0.0.0:1234") {
        Ok(mut stream) => {
            println!(
                "Sucessfully connected to server running at {:?}",
                stream.peer_addr()
            );
            let message = b"Hi how are you";

            stream.write(message).unwrap();
            println!("Send hello to server waiting for reply");

            let mut data = [0 as u8; 14];

            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == message {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                }
                Err(e) => {
                    println!("Failed to receive data from server due to {:}", e);
                }
            }
        }
        Err(e) => {
            println!("Unable to connect with server due to  {:}", e);
        }
    }
}
