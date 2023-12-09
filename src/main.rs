use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn try_read_client_to_string(client: &mut TcpStream) -> Option<String> {
    let mut buffer = [0; 1024];
    match client.read(&mut buffer[..]) {
        Ok(size) => Some(String::from_utf8_lossy(&buffer[..size]).to_string()),
        Err(error) => {
            let peer_addr: String = match client.peer_addr() {
                Ok(addr) => addr.to_string(),
                Err(_) => String::from("Unknown"),
            };
            println!("An error occurred when trying to read peer: {}", peer_addr);
            println!("Error result: {}", error);
            None
        },
    }
}

fn try_write_client(client: &mut TcpStream, response: String) -> Option<usize> {
    match client.write(response.as_bytes()) {
        Ok(size) => Some(size),
        Err(error) => {
            println!("An error occurred when trying to write a response!");
            println!("Error result: {}", error);
            None
        },
    }
}

fn handle_client(client: &mut TcpStream) {
    match try_read_client_to_string(client) {
        Some(_request_string) => {
            let response: String = String::from("HTTP/1.1 200 OK\r\n\r\nteste");
            match try_write_client(client, response) {
                Some(size) => println!("{} bytes have been written successfully!", size),
                None => (),
            };
        },
        None => (),
    }
}

fn main() {
    let addres: &str = "127.0.0.1:4221";
    match TcpListener::bind(addres) {
        Ok(listener) => {
            for connection in listener.incoming() {
                match connection {
                    Ok(mut client_to_handle) => {
                        println!("{:?}", &client_to_handle.peer_addr().unwrap());
                        handle_client(&mut client_to_handle);
                    }
                    Err(e) => {
                        println!("Connection failure in {e:?}");
                    }
                }
            }
        },
        Err(error) => {
            println!("An error occurred when trying to bind addres: {}", addres);
            println!("Error: {}", error);
        },
    }
}