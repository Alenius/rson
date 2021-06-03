use std::{collections::{HashMap, hash_map}, vec} ;
use std::fmt;

#[derive(Clone)]
pub enum JsonValue {
    String(String),
    Num(f64),
    Vec(Vec<JsonValue>), // ok so this works! self referencing
    Bool(bool),
    Object(JsonObject),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(str) => {
                // I think it's clearer that it is a string if 
                // we print it with quotes
                let str_with_quotes = format!("\"{}\"", str);
                return formatter.write_str(&str_with_quotes)
            }
            Self::Num(float) => {
                let output = float.to_string();
                return formatter.write_str(&output)
            }
            Self::Vec(vec) => {
                let mut vec_builder: Vec<String> = vec![];

                for value in vec {
                    vec_builder.push(value.to_string());
                }

                let joined_vec = vec_builder.join(", ");
                let final_vec = format!("[{}]", joined_vec);
                return formatter.write_str(&final_vec)

            }
            Self::Bool(bool) => {
                return formatter.write_str(&bool.to_string());
            }
            Self::Object(obj) => {
                let mut obj_builder: Vec<String> = vec![];

                for (key, val) in obj.to_iter() {
                    obj_builder.push(format!("\"{}\": {}", key, val));
                }

                let joined_obj = obj_builder.join(",");
                let with_brackets = format!("{{{}}}", joined_obj);

                return formatter.write_str(&with_brackets)
            }
        }
    }
}

#[derive(Clone)]
pub struct JsonObject {
    json: HashMap<String, JsonValue>,
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
        while let Some((key, value)) = print_iter.next() {
            println!("  \"{}\": {}", key, value);
        }
        println!("}}");
    }    

    pub fn empty(&mut self) {
        self.json.clear();
    }

    pub fn delete_key(&mut self, key: &str) {
        self.json.remove(key);
    }

    pub fn insert(&mut self, key: String, value: JsonValue) {
        self.json.insert(key, value);
    }

    pub fn get_value(&self, key: &str) -> Option<&JsonValue> {
        return self.json.get(&key.to_owned())
    }

    pub fn to_iter(&self) -> hash_map::Iter<String, JsonValue> {
        return self.json.iter();
    }
}