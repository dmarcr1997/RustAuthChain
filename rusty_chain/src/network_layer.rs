use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

#[derive(Debug, Clone, Copy)]
pub enum Role {
    Creator,
    Authorizer,
    Listener
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let received_data = std::str::from_utf8(&buffer[..size]).unwrap();
            
            match received_data {
                "Creator" => {
                    println!("Creator connected");
                },
                "Authorizer" => {
                    println!("Authorizer connected");
                },
                _ => println!("Unknown role connected"),
            }
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
            return;
        }
    }
}

pub fn start_server(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}