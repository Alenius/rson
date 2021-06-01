use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum JsonValue {
    String(String),
    Num(f64),
    Vec(Vec<JsonValue>), // ok so this works! self referencing
    Bool(bool),
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

    pub fn print(&mut self) {
        let mut print_iter = self.json.clone().into_iter();

        println!("{{");
        while let Some(item) = print_iter.next() {
            println!("{:?}", item);
        }
        println!("}}");
    }    

    pub fn empty(&mut self) {
        self.json.clear();
    }

    pub fn delete_key(&mut self, key: &str) {
        self.json.remove(key);
    }
}