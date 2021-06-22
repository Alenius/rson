use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

use super::annotated_lexer::{Delimiters, JsonTokenType, Token};

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Num(f64),
    Bool(bool),
    Null,
    Vec(Vec<JsonValue>), // ok so this works! self referencing
    Object(JsonObject),
}

#[derive(Debug)]
pub struct JsonObject {
    json: HashMap<String, JsonValue>,
}

impl JsonObject {
    fn new() -> JsonObject {
        return JsonObject {
            json: HashMap::new(),
        };
    }
    fn insert(&mut self, key: String, val: JsonValue) {
        self.json.insert(key, val);
    }
}

pub fn parse_tokens(iter: Iter<Token>) -> (Iter<Token>, JsonObject) {
    let mut token_iter = iter.clone();
    let mut object = JsonObject::new();

    let mut key: Option<String> = None;

    loop {
        if let None = key {
            key = get_key(&token_iter.next());
        }

        // check delimiter
        if let Some(token) = token_iter.next() {
            if let JsonTokenType::Delimiter(Delimiters::Colon) = token.get_token() {
                {}
            } else {
                panic!("Did not get colon after key, instead got: {:?}", token)
            }
        }

        let token_value = token_iter.next().unwrap();

        match token_value.get_token() {
            // string
            JsonTokenType::String(string) => {
                object.insert(key.unwrap(), JsonValue::String(string));
                key = None;
            }
            JsonTokenType::Number(num) => {
                object.insert(key.unwrap(), JsonValue::Num(num));
                key = None;
            }
            JsonTokenType::Boolean(bool) => {
                object.insert(key.unwrap(), JsonValue::Bool(bool));
                key = None;
            }
            JsonTokenType::Null => {
                object.insert(key.unwrap(), JsonValue::Null);
                key = None;
            }
            JsonTokenType::Delimiter(del) => {
                match del {
                    // object open
                    Delimiters::LeftBrace => {
                        let (partly_consumed_iter, nested_object) = parse_tokens(token_iter);
                        token_iter = partly_consumed_iter;
                        object.insert(key.unwrap(), JsonValue::Object(nested_object));
                        return (token_iter, object)
                    }
                    Delimiters::RightBrace => panic!("Stray right brace"),
                    // array
                    Delimiters::LeftBracket => {
                        let mut vec: Vec<JsonValue> = vec![];
                        loop {
                            match token_value.get_token() {
                                JsonTokenType::String(string) => {
                                    vec.push(JsonValue::String(string))
                                }
                                JsonTokenType::Number(num) => vec.push(JsonValue::Num(num)),
                                JsonTokenType::Boolean(bool) => vec.push(JsonValue::Bool(bool)),
                                JsonTokenType::Null => vec.push(JsonValue::Null),
                                JsonTokenType::Delimiter(del) => {
                                    // object inside array
                                    match del {
                                        Delimiters::LeftBrace => {
                                            let (partly_consumed_iter, nested_object) =
                                                parse_tokens(token_iter);
                                            token_iter = partly_consumed_iter;
                                            vec.push(JsonValue::Object(nested_object));

                                            let next_token = token_iter.next();
                                            if let Some(token) = next_token {
                                                    if let JsonTokenType::Delimiter(Delimiters::RightBracket) = token.get_token() {
                                                        return (token_iter, object)
                                                    } else {
                                                        panic!("Array is not finished with right bracket")
                                                    }
                                            }
                                        }
                                        Delimiters::RightBrace => {
                                            // should already have been consumed by the object iter
                                            panic!("Did not expect lonely right brace in array")
                                        }
                                        Delimiters::Comma => {}
                                        _ => panic!("Not implemented nested arrays yet"),
                                    }
                                }
                            }
                        }
                    }
                    Delimiters::RightBracket => panic!("Did not expect a single right bracket"),
                    Delimiters::Comma | Delimiters::Colon => continue,
                }
            }
        }

        let finish_token = token_iter.next();
        if let Some(token) = finish_token {
            if let JsonTokenType::Delimiter(Delimiters::RightBrace) = token.get_token() {
                // end of object
                if let Some(del) = token_iter.next() {
                    if let JsonTokenType::Delimiter(Delimiters::Comma) = del.get_token() {
                        return (token_iter, object) 
                    } else {
                        panic!("Expected comma or nothing after object")
                    }
                }
            }
            if let JsonTokenType::Delimiter(Delimiters::Comma) = token.get_token() {
                continue;
            } else {
                panic!("Key value pair did not end in comma or end of input: {:?}", token)
            }
        } else {
            // token_iter will be depleted so this might be handled better
            return (token_iter, object);
        }
    }
}

pub fn annotated_parser(insert_vec: Vec<Token>) -> JsonObject {
    let mut token_vec = insert_vec.clone();

    let first_token = token_vec.remove(0);
    if first_token
        .get_token()
        .ne(&JsonTokenType::Delimiter(Delimiters::LeftBrace))
    {
        panic!(
            "Object does not start with left brace, instead I got: {:?}",
            first_token
        );
    }
    let last_token = token_vec.pop();
    if let Some(last_token) = last_token {
        if last_token
            .get_token()
            .ne(&JsonTokenType::Delimiter(Delimiters::RightBrace))
        {
            panic!(
                "Object does not end with right brace, instead I got: {:?}",
                first_token
            );
        }
    }

    let token_iter = token_vec.iter();
    let (_, json_object) = parse_tokens(token_iter);

    return json_object;
}

fn get_key(next_val: &Option<&Token>) -> Option<String> {
    if let Some(next_key) = next_val {
        let key = next_key.get_token();
        if let JsonTokenType::String(val) = key {
            return Some(val);
        } else {
            panic!("Expected key to be string, instead got: {:?}", key)
        }
    } else {
        panic!("Unexpected end of object");
    }
}
