use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::database::{StorageEngine};
use crate::query::{ QueryEngine };
use crate::server::tcp_server::start_tcp_server;
use crate::utils::demo_data::autosave_database;

pub fn run_cli(storage: Arc<Mutex<StorageEngine>>) {
    println!("atomDB CLI Mode");
    println!("Type 'help' for commands or 'quit' to exit.");
    println!("Database autosaves after every write operation.");
    
    loop {
        print!("atomDB> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            let storage_guard = storage.lock().unwrap();
            autosave_database(&storage_guard);
            println!("Final autosave completed. Goodbye!");
            break;
        }
        
        if input.eq_ignore_ascii_case("help") {
            show_help();
            continue;
        }

        if input.eq_ignore_ascii_case("server") {
            println!("Starting TCP server...");
            let storage_clone = Arc::clone(&storage);
            thread::spawn(move || {
                start_tcp_server(storage_clone);
            });
            println!("TCP server started on port 4000. You can now connect via TCP.");
            continue;
        }
        
        if input.eq_ignore_ascii_case("load") {
            handle_load_command(&storage);
            continue;
        }
        
        execute_query(&storage, input);
    }
}

fn show_help() {
    println!(r#"
Available Commands:
  CREATE TABLE <name> (col1, col2, ...)  - Create a new table
  INSERT INTO <table> (col1=val1, ...)   - Insert a row
  SELECT * FROM <table>                  - Select all rows
  SELECT * FROM <table> WHERE col=val    - Select with condition
  DESCRIBE <table>                       - Show table columns
  SHOW TABLES                           - List all tables
  DROP TABLE <table>                    - Delete a table
  LOAD                                  - Load database from binary file
  SERVER                                - Start TCP server mode
  quit/exit                             - Exit the CLI

Note: Database autosaves after every write operation
"#);
}

fn handle_load_command(storage: &Arc<Mutex<StorageEngine>>) {
    let result = StorageEngine::load_from_binary_file("database.bin");
    match result {
        Ok(loaded_storage) => {
            *storage.lock().unwrap() = loaded_storage;
            println!("Database loaded successfully");
        },
        Err(e) => println!("Error loading: {}", e),
    }
}

fn execute_query(storage: &Arc<Mutex<StorageEngine>>, input: &str) {
    let is_write_operation = input.to_lowercase().starts_with("create") || 
                           input.to_lowercase().starts_with("insert") || 
                           input.to_lowercase().starts_with("drop");
    
    let response = {
        let mut storage_guard = storage.lock().unwrap();
        let mut query_engine = QueryEngine::new(&mut storage_guard);
        let result = query_engine.execute(input);
        
        if is_write_operation && result.is_ok() {
            autosave_database(&storage_guard);
            println!("Database autosaved");
        }
        
        result
    };
    
    match response {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }
}
