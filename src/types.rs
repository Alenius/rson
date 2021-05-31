use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum JsonValue {
    String(String),
    Num(f64),
    Vec(Vec<JsonValue>), // ok so this works! self referencing
    Bool(bool),
    // Object(HashMap<&'a String, JsonValue<'a>>),
    Object(JsonObject),
}

#[derive(Debug, Clone)]
pub struct JsonObject {
    pub json: HashMap<String, JsonValue>,
}

impl JsonObject {
    pub fn new() -> Self {
        return JsonObject {
            json: HashMap::new(),
        };
    }
}