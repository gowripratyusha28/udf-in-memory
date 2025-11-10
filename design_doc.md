# User Defined Function Database 

## 1. Overview
Build an in-memory document database, with an embedded engine for running user-defined functions (”UDF”)

Problem statement link: https://erik-dunteman.notion.site/Coding-Project-User-Defined-Functions-2a0d787b41e98027ab86ec475c265d1f

## 2. Choices made:
### Host language: Rust  systems language
- Makes single binary trivial
- Considered C++ as I am very comfortable with it but build is complex and wanted to get my hands dirty with Rust

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

## 3. References:
- https://duckdb.org/docs/stable/clients/python/function
- https://spacetimedb.com/docs/?syntax=rust
- https://15721.courses.cs.cmu.edu/spring2024/notes/11-udfs.pdf
- https://medium.com/%40mwendakelvinblog/lua-the-language-for-embedded-applications-and-cross-platform-use-8e0817581bb7
- https://www.boringcactus.com/2020/09/16/survey-of-rust-embeddable-scripting-languages.html


## 4. Follow up question on this design:
Are we keeping the data as active Lua objects in memory? Specifically, does each object live within its own Lua runtime, or do they share a common namespace? I’m trying to understand the trade offs here, how lightweight are Lua objects, and do they inherently require a runtime context? Alternatively, could a single memory region be owned by Rust (without an active Lua runtime) and later reinterpreted as Lua objects on demand?

## 5. Memory & Runtime Strategy Choices:
- Lua tables must exist inside a Lua runtime(cannot exist without runtime)

### Option 1 (Store as Rust Objects):
- The data is all stored as rust objects
- Query flow: for each row, rust value -> (convert) -> Lua table -> UDF -> Result
- Safe memory management
- Con: Need to convert Rust to Lua for every query (As number of rows increases timing increases)

### Option 2 (Store as Lua Objects):
- Data is stored in Lua runtime
- All the data is stored in single runtime
- Zero conversion cost for every query
- Con: Data lifetime is tied to Lua runtime
- Updates can get complex

### Option 3 (Hybrid):
- I was thinking something like cache combining the above 2 approach
- Hybrid choice 1: Caching query results will help repeated queries return the results fast. Works well for read heavy workloads. But if there are many updates(write heavy) then this doesn't help or if the queries are very unique this approach will not help.
- Hybrid choice 2: Cache few lua table (maybe something like LRU logic). But need to know what kind of queries are asked and will they access all the documents everytime or not. Cache invalidation logic is also needed.
- Need to know the type of workload and the type of query for considering any kind of hybrid approach.

### Other optimizations:
- Parallel filtering using multiple threads (Each thread has its own Lua runtime)


I would consider option 1 for now and then move onto option 3 in future based on constraints.
