pub fn get_test_documents() -> Vec<(&'static str, &'static str)> {
    vec![
        ("1", r#"{"message": "hello world", "priority": 5}"#),
        ("2", r#"{"foo": "bar"}"#),
        ("3", r#"{"message": "HELLO", "active": true, "priority": 9}"#),
        ("4", r#"{"active": false}"#),
        ("5", r#"{"message": "bonjour", "priority": 7}"#),
    ]
}

pub struct UdfDefinition {
    pub name: &'static str,
    pub code: &'static str,
    pub description: &'static str,
}

pub fn get_test_udfs() -> Vec<UdfDefinition> {
    vec![
        UdfDefinition {
            name: "has_hello",
            description: "Documents containing 'hello' (case-insensitive)",
            code: r#"
                function(val)
                    if val.message == nil then
                        return false
                    end
                    return string.lower(val.message):find("hello") ~= nil
                end
            "#,
        },
        UdfDefinition {
            name: "high_priority",
            description: "Documents with priority >= 8",
            code: r#"
                function(val)
                    return val.priority and val.priority >= 8
                end
            "#,
        },
        UdfDefinition {
            name: "is_active",
            description: "Documents where active == true",
            code: r#"
                function(val)
                    return val.active == true
                end
            "#,
        },
    ]
}