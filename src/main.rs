use std::collections::HashMap;
use serde_json::Value as JsonValue;
use mlua::{Lua, Function, LuaSerdeExt};

struct Database {
    store: HashMap<String, JsonValue>,
    compiled_udfs: HashMap<String, String>,
    lua: Lua,
}

impl Database {
    fn new() -> Self {
        Database {
            store: HashMap::new(),
            compiled_udfs: HashMap::new(),
            lua: Lua::new(),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), String> {
        let parsed: JsonValue = serde_json::from_str(&value)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        self.store.insert(key, parsed);
        
        Ok(())
    }

    fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).map(|v| v.to_string())
    }

    fn register_udf(&mut self, name: String, code: String) -> Result<(), String> {
        self.lua.load(&code).eval::<Function>()
            .map_err(|e| format!("Invalid Lua code: {}", e))?;

        self.compiled_udfs.insert(name, code);
        
        Ok(())
    }

    fn get_where(&self, udf_name: &str) -> Result<Vec<(String, String)>, String> {
        let udf_code = self.compiled_udfs.get(udf_name).ok_or_else(|| format!("UDF '{}' not found", udf_name))?;

        let lua_func = self
            .lua
            .load(udf_code)
            .eval::<Function>()
            .map_err(|e| format!("Failed to load UDF '{}': {}", udf_name, e))?;

        let mut results = Vec::new();

        for (key, value) in &self.store {
            let lua_value = self.lua.to_value(value)
            .map_err(|e| format!("Failed to convert to Lua: {}", e))?;

                match lua_func.call::<_, bool>(lua_value) {
                    Ok(true) => {
                        // UDF returned true, include this row
                        results.push((key.clone(), value.to_string()));
                    }
                    Ok(false) => {
                        // UDF returned false, skip
                    }
                    Err(e) => {
                        // UDF error, log and continue
                        eprintln!("UDF error for key '{}': {}", key, e);
                    }
                }
        }

        Ok(results)
    }

    fn keys(&self) -> impl Iterator<Item = &String> {
        self.store.keys()
    }

    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    fn list_udfs(&self) -> impl Iterator<Item = &String> {
        self.compiled_udfs.keys()
    }
}

fn main() {
    let mut db = Database::new();

    println!("=================== Setting up test data ===================");

    let test_data = vec![
        ("1", r#"{"message": "hello world", "priority": 5}"#),
        ("2", r#"{"foo": "bar"}"#),
        ("3", r#"{"message": "HELLO", "active": true, "priority": 9}"#),
        ("4", r#"{"active": false}"#),
        ("5", r#"{"message": "Bonjour", "priority": 3}"#),
    ];

    for (key, value) in test_data {
        match db.set(key.to_string(), value.to_string()) {
            Ok(_) => println!("Inserted key '{}'", key),
            Err(e) => eprintln!("Failed to insert '{}': {}", key, e),
        }
    }

    println!("\n=================== Current database contents ===================");
    for key in db.keys() {
        if let Some(value) = db.get(key) {
            println!("  {}: {}", key, value);
        }
    }

    println!("\n=================== Registering UDF ===================");
    let udf_code = r#"
        function(val)
            if val.message == nil then
                return false
            end
            return string.lower(val.message):find("hello") ~= nil
        end
    "#;

    match db.register_udf("has_hello".to_string(), udf_code.to_string()) {
        Ok(_) => println!("Registered UDF 'has_hello'"),
        Err(e) => {
            eprintln!("Failed to register UDF: {}", e);
        }
    }

    let high_priority_udf = r#"
        function(val)
            return val.priority and val.priority >= 8
        end
    "#;

    match db.register_udf("high_priority".to_string(), high_priority_udf.to_string()) {
        Ok(_) => println!("Registered UDF 'high_priority'"),
        Err(e) => {
            eprintln!("Failed to register UDF: {}", e);
        }
    }

    let is_active_udf = r#"
        function(val)
            return val.active == true
        end
    "#;

    match db.register_udf("is_active".to_string(), is_active_udf.to_string()) {
        Ok(_) => println!("Registered UDF 'is_active'"),
        Err(e) => eprintln!("Failed to register UDF: {}", e),
    }

    println!("\n=================== Registered UDFs =================== ");
    for udf_name in db.list_udfs() {
        println!("  - {}", udf_name);
    }

    println!("\n=================== [get_where] Query with UDF ===================");

    println!("\n[Query: has_hello]");
    match db.get_where("has_hello") {
        Ok(results) => {
            println!("Found {} matching document(s):", results.len());
            for (key, value) in results {
                println!("  Key: {}, Value: {}", key, value);
            }
        }
        Err(e) => eprintln!("Query failed: {}", e),
    }

    println!("\n[Query: high_priority]");
    match db.get_where("high_priority") {
        Ok(results) => {
            println!("Found {} matching document(s):", results.len());
            for (key, value) in results {
                println!("  Key: {}, Value: {}", key, value);
            }
        }
        Err(e) => eprintln!("Query failed: {}", e),
    }

    println!("\n[Query: is_active]");
    match db.get_where("is_active") {
        Ok(results) => {
            println!("Found {} matching document(s):", results.len());
            for (key, value) in results {
                println!("  Key: {}, Value: {}", key, value);
            }
        }
        Err(e) => eprintln!("Query failed: {}", e),
    }

    println!("\n[Query: nonexistent_udf]");
    match db.get_where("nonexistent_udf") {
        Ok(results) => {
            println!("Found {} matching document(s):", results.len());
            for (key, value) in results {
                println!("  Key: {}, Value: {}", key, value);
            }
        }
        Err(e) => eprintln!("Query failed: {}", e),
    }

    println!("\n=================== Summary ===================");
    println!("Total documents: {}", db.len());
    println!("Registered UDFs: {}", db.list_udfs().count());
}