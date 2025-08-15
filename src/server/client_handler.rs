use std::io::{Write, BufRead, BufReader};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::database::{StorageEngine };
use crate::query::{ QueryEngine };
use crate::utils::demo_data::autosave_database;

pub fn handle_tcp_client(mut stream: TcpStream, storage: Arc<Mutex<StorageEngine>>) {
    let peer = stream.peer_addr().unwrap();
    println!("TCP Client connected: {}", peer);

    let _ = writeln!(stream, "Welcome to atomDB TCP Server!");
    let _ = writeln!(stream, "Type SQL commands or 'help' for assistance. 'quit' to disconnect.");

    let reader = BufReader::new(stream.try_clone().unwrap());
    
    for line in reader.lines() {
        match line {
            Ok(query) => {
                let query = query.trim();
                
                if query.is_empty() {
                    continue;
                }
                
                if query.eq_ignore_ascii_case("quit") {
                    let _ = writeln!(stream, "Goodbye!");
                    break;
                }
                
                if query.eq_ignore_ascii_case("help") {
                    let help_text = r#"
Available Commands:
  CREATE TABLE <name> (col1, col2, ...)
  INSERT INTO <table> (col1=val1, ...)
  SELECT * FROM <table>
  SELECT * FROM <table> WHERE col=val
  DESCRIBE <table>
  SHOW TABLES
  DROP TABLE <table>
  LOAD
  quit

Note: Database autosaves after every write operation
"#;
                    let _ = write!(stream, "{}", help_text);
                    continue;
                }

                if query.eq_ignore_ascii_case("load") {
                    let result = StorageEngine::load_from_binary_file("database.bin");
                    let response = match result {
                        Ok(loaded_storage) => {
                            *storage.lock().unwrap() = loaded_storage;
                            "Database loaded successfully".to_string()
                        },
                        Err(e) => format!("Error loading: {}", e),
                    };
                    let _ = writeln!(stream, "{}", response);
                    continue;
                }

                let is_write_operation = query.starts_with("create") || 
                                       query.starts_with("insert") || 
                                       query.starts_with("drop");

                let response = {
                    let mut storage_guard = storage.lock().unwrap();
                    let mut query_engine = QueryEngine::new(&mut storage_guard);
                    let result = match query_engine.execute(query) {
                        Ok(result) => result,
                        Err(err) => format!("Error: {}", err),
                    };
                    
                    if is_write_operation {
                        autosave_database(&storage_guard);
                        println!("Database autosaved after write operation");
                    }
                    
                    result
                };

                let _ = writeln!(stream, "{}", response);
            },
            Err(e) => {
                eprintln!("Error reading from TCP client {}: {}", peer, e);
                break;
            }
        }
    }

    println!("TCP Client {} disconnected", peer);
}
