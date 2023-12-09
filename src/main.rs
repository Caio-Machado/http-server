use std::net::TcpListener;

mod connection;

fn main() {
    let addres: &str = "127.0.0.1:4221";
    match TcpListener::bind(addres) {
        Ok(listener) => {
            for connection in listener.incoming() {
                match connection {
                    Ok(mut client_to_handle) => {
                        connection::handle_client(&mut client_to_handle);
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