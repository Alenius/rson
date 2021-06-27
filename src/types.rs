use std::fmt;
use std::{
    collections::{hash_map, HashMap},
    vec,
};

#[derive(Clone, Debug, PartialEq)]
pub enum JsonNum {
    Int(i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Num(JsonNum),
    Bool(bool),
    Null,
    Vec(Vec<JsonValue>), // ok so this works! self referencing
    Object(JsonObject),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(str) => {
                // I think it's clearer that it is a string if
                // we print it with quotes
                let str_with_quotes = format!("\"{}\"", str);
                return formatter.write_str(&str_with_quotes);
            }
            Self::Num(num) => match num {
                &JsonNum::Float(float) => {
                    let output = float.to_string();
                    return formatter.write_str(&output);
                }
                &JsonNum::Int(int) => {
                    let output = int.to_string();
                    return formatter.write_str(&output);
                }
            },
            Self::Vec(vec) => {
                let mut vec_builder: Vec<String> = vec![];

                for value in vec {
                    vec_builder.push(value.to_string());
                }

                let joined_vec = vec_builder.join(", ");
                let final_vec = format!("[{}]", joined_vec);
                return formatter.write_str(&final_vec);
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

                return formatter.write_str(&with_brackets);
            }
            Self::Null => {
                return formatter.write_str("null")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
        return self.json.get(key);
    }

    pub fn to_iter(&self) -> hash_map::Iter<String, JsonValue> {
        return self.json.iter();
    }

    pub fn get_string_value(&self, key: &str ) -> Option<&String> {

        let val = self.json.get(key);
        if val.is_none() {
            return None;
        }

        let val = val.unwrap();
        match val {
            JsonValue::String(val) => {
                return Some(val);
            }
            unexpected => {
                panic!("The value for that key is not of type String, instead it is: {:?}", unexpected)
            }
        }
    } 

    pub fn get_keys(&self) -> Vec<&String> {
        let raw_keys = self.json.keys().into_iter();
        let mut keys: Vec<&String> = vec![];

        raw_keys.for_each(|x| keys.push(x));
        return keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value() {
        let mut obj = JsonObject::new(); 
        obj.insert("str".to_owned(), JsonValue::String("String".to_owned()));

        let val = obj.get_value("str").unwrap();
        assert_eq!(val, &JsonValue::String("String".to_owned()));
    }

    #[test]
    fn test_get_string_value() {
        let mut obj = JsonObject::new(); 
        obj.insert("str".to_owned(), JsonValue::String("String".to_owned()));

        let string = obj.get_string_value("str").unwrap();
        assert_eq!(string, "String");
    }

    #[test]
    #[should_panic(expected = "The value for that key is not of type String, instead it is: Bool(false)")]
    fn test_get_string_value_faulty() {
        let mut obj = JsonObject::new(); 
        obj.insert("str".to_owned(), JsonValue::Bool(false));

        obj.get_string_value("str").unwrap();
    }

    #[test]
    fn test_get_keys() {
        let mut obj = JsonObject::new();
        obj.insert("true".to_owned(), JsonValue::Bool(true));
        obj.insert("false".to_owned(), JsonValue::Bool(false));

        let keys = obj.get_keys();
        assert_eq!(keys, vec![&"true".to_owned(), &"false".to_owned()]);
    }
}