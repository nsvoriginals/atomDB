use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use atom::{StorageEngine, run_cli, start_tcp_server, setup_demo_data};

fn show_usage() {
    println!("atomDB - A Rust Database with Immediate Autosave");
    println!("Usage:");
    println!("  cargo run                    - Start CLI mode");
    println!("  cargo run -- --server        - Start TCP server mode");
    println!("  cargo run -- --both          - Start both CLI and TCP server");
    println!("  cargo run -- --help          - Show this help");
    println!();
    println!("Features:");
    println!("  - Autosaves immediately after every write operation");
    println!("  - Final autosave on exit");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let initial_storage = match StorageEngine::load_from_binary_file("database.bin") {
        Ok(storage) => {
            println!("Loaded existing database from binary file");
            storage
        },
        Err(_) => {
            println!("Creating new database with demo data");
            let mut storage = StorageEngine::new();
            setup_demo_data(&mut storage).expect("Failed to setup demo data");
            storage
        }
    };
    
    let storage = Arc::new(Mutex::new(initial_storage));
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                show_usage();
                return Ok(());
            },
            "--server" | "-s" => {
                println!("Starting TCP server mode only...");
                start_tcp_server(storage);
            },
            "--both" | "-b" => {
                println!("Starting both CLI and TCP server...");
                let storage_for_server = Arc::clone(&storage);
                thread::spawn(move || {
                    start_tcp_server(storage_for_server);
                });
                thread::sleep(Duration::from_millis(100));
                run_cli(storage);
            },
            _ => {
                println!("Unknown argument: {}", args[1]);
                show_usage();
            }
        }
    } else {
        run_cli(storage);
    }
    
    Ok(())
}
