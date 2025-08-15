use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::database::StorageEngine;
use super::client_handler::handle_tcp_client;

pub fn start_tcp_server(storage: Arc<Mutex<StorageEngine>>) {
    let listener = TcpListener::bind("0.0.0.0:6969").expect("Failed to bind TCP server");
    println!("TCP Server listening on 0.0.0.0:6969");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let storage_clone = Arc::clone(&storage);
                thread::spawn(move || {
                    handle_tcp_client(stream, storage_clone);
                });
            },
            Err(e) => {
                eprintln!("Failed to accept TCP connection: {}", e);
            }
        }
    }
}
