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
    let response_200: String = String::from("HTTP/1.1 200 OK\r\n\r\n");
    let response_404: String = String::from("HTTP/1.1 404 Not Found\r\n\r\n");
    match try_read_client_to_string(client) {
        Some(request_string) => {
            // println!("{:?}", request_string);
            let request_path: Result<String, ()> = match request_string.split_once("\r\n") {
                Some((start_line, _)) => {
                    let splited_start_line: Vec<&str> = start_line.split_whitespace().collect();
                    match splited_start_line.get(1) {
                        Some(path) => Ok(path.to_string()),
                        None => Err(()),
                    }
                },
                None => Err(()),
            };

            match request_path {
                Ok(path) => {
                    match path.as_str() {
                        "/" => {
                            match try_write_client(client, response_200) {
                                Some(size) => println!("{} bytes have been written successfully!", size),
                                None => (),
                            };
                        },
                        _ => {
                            match try_write_client(client, response_404) {
                                Some(size) => println!("{} bytes have been written successfully!", size),
                                None => (),
                            };
                        }
                    }
                },
                Err(_) => (),
            }
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