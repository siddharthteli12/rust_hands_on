use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.0:1234").unwrap();

    println!("Server started & listening on 1234 port");

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                println!("Added new connection");
                thread::spawn(move || {
                    handle_client(tcp_stream);
                });
            }
            Err(e) => {
                eprintln!("Error {:}", e);
            }
        }
    }
    // Close the socket server,
    drop(listener);
}

fn handle_client(mut tcp_stream: TcpStream) {
    // Buffer array,
    let mut data = [0 as u8; 50];
    while match tcp_stream.read(&mut data) {
        Ok(size) => {
            tcp_stream.write(&data[0..size]).unwrap();
            true
        }
        Err(e) => {
            println!(
                "Error occurred, closing connection with {:}, due to {:}",
                tcp_stream.peer_addr().unwrap(),
                e
            );
            tcp_stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
