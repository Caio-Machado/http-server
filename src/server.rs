use std::collections::HashMap;
use std::io::prelude::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use self::request::Request;
use self::response::Response;

mod header;
mod request;
mod response;

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
        }
    }
}

fn try_write_client(client: &mut TcpStream, response: String) -> Option<usize> {
    match client.write(response.as_bytes()) {
        Ok(size) => Some(size),
        Err(error) => {
            println!("An error occurred when trying to write a response!");
            println!("Error result: {}", error);
            None
        }
    }
}

fn handle_client(client: &mut TcpStream) {
    match try_read_client_to_string(client) {
        Some(request_string) => {
            let request: Option<Request> = match Request::extract_request_fields(request_string) {
                Ok((start_line, headers, body)) => Some(Request::new(start_line, headers, body)),
                Err(_) => None,
            };
            match request {
                Some(r) => {
                    let response: Response = match r.start_line.request_target.path.as_str() {
                        "echo" => {
                            let body = r.start_line.request_target.random_string;
                            let headers: HashMap<&str, String> = HashMap::from([
                                ("Content-Type", String::from("text/plain")),
                                (
                                    "Content-Length",
                                    format!("{}", body.as_ref().unwrap().len()),
                                ),
                            ]);
                            match Response::try_build_response_fields(
                                200,
                                Some(headers),
                                body.as_ref(),
                            ) {
                                Ok((status_line, headers, body)) => {
                                    Response::build_response(status_line, headers, body)
                                }
                                Err(_) => return (),
                            }
                        }
                        "/" => match Response::try_build_response_fields(200, None, None) {
                            Ok((status_line, headers, body)) => {
                                Response::build_response(status_line, headers, body)
                            }
                            Err(_) => return (),
                        },
                        _ => match Response::try_build_response_fields(404, None, None) {
                            Ok((status_line, headers, body)) => {
                                Response::build_response(status_line, headers, body)
                            }
                            Err(_) => return (),
                        },
                    };
                    match try_write_client(client, response.build_response_as_string()) {
                        Some(size) => println!("{size} bytes have been written successfully"),
                        None => (),
                    }
                }
                None => (),
            }
        }
        None => (),
    }
}

pub fn run(addres: &str) {
    match TcpListener::bind(addres) {
        Ok(listener) => loop {
            match listener.accept() {
                Ok((mut client_to_handle, _)) => {
                    let handler = thread::spawn(move || handle_client(&mut client_to_handle));
                    handler.join().unwrap();
                }
                Err(e) => {
                    println!("Connection failure in {e:?}");
                }
            }
        },
        Err(error) => {
            println!("An error occurred when trying to bind addres: {}", addres);
            println!("Error: {}", error);
        }
    }
}
