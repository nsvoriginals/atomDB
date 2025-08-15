# AtomDB

**AtomDB** is a high-performance, in-memory database written in Rust.  
It features SQL-like query capabilities, TCP server support, and automatic persistence — all in a safe and efficient architecture.

---

##  Features

-  **High-performance** in-memory operations powered by Rust
-  **SQL-like** query syntax for familiar database interactions
-  Dual storage formats: **JSON** and **binary** serialization
-  Automatic persistence after every write operation
-  **TCP server** for remote access
-  Interactive **command-line interface** for local usage
-  **Docker** containerization support
- Memory-safe architecture using Rust’s ownership system
-  Multi-threaded TCP server for concurrent connections

---

## 🚀 Quick Start

### Prerequisites
- **Rust** 1.75 or later
- **Docker** (optional, for containerized deployment)

### Installation
```bash
git clone https://github.com/yourusername/atomdb.git
cd atomdb
cargo build --release
```

### Basic Usage
**Start CLI mode**
```bash
cargo run
```

**Start TCP server only**
```bash
cargo run -- --server
```

**Start both CLI and TCP server**
```bash
cargo run -- --both
```

---

##  Command Reference

### Table Operations
```
CREATE TABLE table_name (column1, column2, column3)
DROP TABLE table_name
SHOW TABLES
DESCRIBE table_name
```

### Data Operations
```
INSERT INTO table_name (col1=value1, col2=value2)
SELECT * FROM table_name
SELECT * FROM table_name WHERE column=value
```

### System Commands
```
LOAD
SERVER
quit
```

---

##  Network Access

By default, AtomDB runs its TCP server on **port 6969**.

### Connect via Telnet
```bash
telnet localhost 6969
```

### Python Client Example
```python
import socket

client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect(('localhost', 6969))
client.send(b"SELECT * FROM users\n")
response = client.recv(4096).decode()
print(response)
client.close()
```

### Rust Client Example
```rust
use std::net::TcpStream;
use std::io::{Write, BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6969")?;
    writeln!(stream, "SELECT * FROM users")?;

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("Response: {}", response);
    Ok(())
}
```

---

##  Docker Deployment

### Using Docker Compose
```bash
docker-compose up --build -d
docker-compose logs -f atomdb
```

### Using Docker Directly
```bash
docker build -t atomdb:latest .
docker run -d --name atomdb-server   -p 6969:6969   -v atomdb-data:/app/data   atomdb:latest
```

---

##  Project Structure
```
atomdb/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── database/
│   │   ├── mod.rs
│   │   ├── storage.rs
│   │   └── schema.rs
│   ├── query/
│   │   ├── mod.rs
│   │   └── parser.rs
│   ├── server/
│   │   ├── mod.rs
│   │   ├── tcp_server.rs
│   │   └── client_handler.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   └── interface.rs
│   └── utils/
│       ├── mod.rs
│       └── demo_data.rs
├── Dockerfile
├── docker-compose.yml
└── Cargo.toml
```

---

## ⚙ Configuration

### Environment Variables
```env
RUST_LOG=info # Logging level
DATABASE_PATH=/app/data/database.bin # Database file location
```

### Dependencies
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
```

---

##  Architecture

- **Storage Engine** – Handles data persistence & retrieval  
- **Query Engine** – Processes SQL-like commands  
- **TCP Server** – Manages network connections  
- **CLI Interface** – Interactive local usage  
- **Schema Management** – Defines tables and rows  

---

##  Performance
- In-memory operations with **sub-millisecond** query times  
- Binary serialization for efficient disk I/O  
- Multi-threaded TCP server for concurrent connections  
- Automatic persistence for durability  
- Zero-copy operations using Rust's ownership model  

---

##  Testing
Run unit tests:
```bash
cargo test
```

With logging:
```bash
RUST_LOG=debug cargo test
```

Run benchmarks:
```bash
cargo bench
```

---

## Troubleshooting

### Connection Issues
Check if the server is running:
```bash
netstat -tulpn | grep 6969
```
Kill processes using the port:
```bash
sudo lsof -ti:6969 | xargs kill -9
```

### Build Issues
```bash
cargo update
cargo clean && cargo build
```

---

##  Development

Set up a development environment:
```bash
git clone https://github.com/nsvoriginals/atomdb.git
cd atomdb
cargo install cargo-watch
cargo watch -x run
```

### Code Style
```bash
cargo fmt
cargo clippy
```

---

##  Roadmap
- [ ] Index support for faster queries  
- [ ] Table join operations  
- [ ] Transaction support with ACID properties  
- [ ] Master-slave replication  
- [ ] REST API interface  
- [ ] User authentication & authorization  
- [ ] Distributed clustering  
- [ ] Web-based admin interface  

---

##  License
This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

##  Support
- Report issues via **GitHub Issues**  
- Join discussions on **GitHub Discussions**  
- Contact: `your.email@example.com`

---

##  Acknowledgments
Built with Rust and powered by:
- **Serde** – Serialization
- **Bincode** – Binary encoding
- **Standard Library** – Networking

---
