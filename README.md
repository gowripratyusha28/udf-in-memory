# udf-in-memory
An in-memory document database with embedded UDF (user-defined function) support

## What it does  
This project provides:  
- A simple in-memory key value store where values are JSON documents.  
- Support for registering UDFs in Lua, which can filter documents by arbitrary predicate logic.  
- A single binary implementation in Rust, embedding a Lua runtime (`mlua`) to execute UDFs.

## Getting Started  
### Prerequisites  
- Rust toolchain (version 1.x or later)  
- (Optional) Lua support is embedded, so no separate Lua installation required.
### Build & Run  
```bash
# Clone the repository
git clone https://github.com/your-username/udf-in-memory.git
cd udf-in-memory

# Build in debug mode (for development)
cargo run

# Or build release binary for production
cargo build --release
./target/release/udf-in-memory
```


See design_doc.md for full details of design choices