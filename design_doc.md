# User Defined Function Database 

## 1. Overview
Build an in-memory document database, with an embedded engine for running user-defined functions (”UDF”)

## 2. Choices made:
### Host language: Rust  systems language
- Makes single binary trivial
- Considered C++ as I am completely comfortable with it but build is complex and wanted to get my hands dirty with Rust

### UDF language: Lua
- Highly used for embedding
- Simple type system, easy JSON conversion
- Easy to bind with Rust
- Heard about Lua when I was in Amazon
- Considered JavaScript (but larger binary)

### JSON storage: Parse on insert rather than parse on filter
- Given: the UDF accepts a dict, meaning the DB pre-deserialized the json value
- With this, there is only cost once for inserting
- During filtering there is no cost
- UDF will work with objects(ex: dict) and not strings
- High memory usage is trade-off

### Register UDF:
- Compile once, execute many times (performance)
- Similar to DuckDB

### Data structures:
- Hashmap to store and get
- Values are stored as parsed serde_json::Value
- Retrieval is almost O(1)

### Limitations:
- Single threaded execution
- O(n) scan for every query (check every element)
- No persistence, deletions, indexing (out of scope)