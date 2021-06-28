use std::slice::Iter;

use crate::types::strip_quotes;

use super::lexer::{Delimiters, JsonTokenType, Numbers, Token};
use super::types::{JsonNum, JsonObject, JsonValue};

fn check_colon_delimiter(token: Option<&Token>) {
    if let Some(token) = token {
        if let JsonTokenType::Delimiter(Delimiters::Colon) = token.get_token() {
            {}
        } else {
            panic!("Did not get colon after key, instead got: {:?}", token)
        }
    } else {
        panic!("Unexpected end of iter")
    }
}

fn parse_array(mut token_iter: Iter<Token>) -> (Iter<Token>, JsonValue) {
    let mut vec: Vec<JsonValue> = vec![];
    loop {
        let arr_val = token_iter.next();

        if arr_val.is_none() {
            panic!("Unexpected end of array");
        }
        match arr_val.unwrap().get_token() {
            JsonTokenType::String(string) => vec.push(JsonValue::String(string)),
            JsonTokenType::Number(num) => match num {
                Numbers::Integer(integer) => vec.push(JsonValue::Num(JsonNum::Int(integer))),
                Numbers::Float(float) => vec.push(JsonValue::Num(JsonNum::Float(float))),
            },
            JsonTokenType::Boolean(bool) => vec.push(JsonValue::Bool(bool)),
            JsonTokenType::Null => vec.push(JsonValue::Null),
            JsonTokenType::Delimiter(del) => {
                match del {
                    Delimiters::Comma => continue,
                    Delimiters::RightBracket => {
                        return (token_iter, JsonValue::Vec(vec));
                    }
                    // nested array
                    Delimiters::LeftBracket => {
                        let (partly_consumed_iter, nested_array) = parse_array(token_iter);
                        token_iter = partly_consumed_iter;
                        vec.push(nested_array);
                    }
                    // object inside array
                    Delimiters::LeftBrace => {
                        let (partly_consumed_iter, nested_object) = parse_tokens(token_iter);
                        token_iter = partly_consumed_iter;
                        vec.push(JsonValue::Object(nested_object));
                    }
                    Delimiters::RightBrace => {
                        // should already have been consumed by the object iter
                        panic!("Did not expect lonely right brace in array")
                    }
                    unexpected => {
                        panic!("Unexpected token: {:?}", unexpected)
                    }
                }
            }
        }
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

        check_colon_delimiter(token_iter.next());

        let token_value = token_iter.next().unwrap();
        match token_value.get_token() {
            JsonTokenType::String(string) => {
                object.insert(key.unwrap(), JsonValue::String(string));
                key = None;
            }
            JsonTokenType::Number(num) => {
                match num {
                    Numbers::Integer(integer) => {
                        object.insert(key.unwrap(), JsonValue::Num(JsonNum::Int(integer)))
                    }
                    Numbers::Float(float) => {
                        object.insert(key.unwrap(), JsonValue::Num(JsonNum::Float(float)))
                    }
                }
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
                        key = None;
                    }
                    Delimiters::RightBrace => panic!("Stray right brace"),
                    // array
                    Delimiters::LeftBracket => {
                        let (partly_consumed_iter, json_value) = parse_array(token_iter);
                        token_iter = partly_consumed_iter;
                        object.insert(key.unwrap(), json_value);
                        key = None;
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
                return (token_iter, object);
            }
            if let JsonTokenType::Delimiter(Delimiters::Comma) = token.get_token() {
                continue;
            } else {
                panic!(
                    "Key value pair did not end in comma or end of input: {:?}",
                    token
                )
            }
        } else {
            // token_iter will be depleted so this might be handled better
            return (token_iter, object);
        }
    }
}

fn remove_first_and_last_brace(mut token_vec: Vec<Token>) -> Vec<Token> {
    if token_vec.len() == 0 {
        return token_vec;
    }

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
                last_token
            );
        }
    }

    return token_vec;
}

pub fn parse(insert_vec: Vec<Token>) -> JsonObject {
    let mut token_vec = insert_vec.clone();

    token_vec = remove_first_and_last_brace(token_vec);

    let token_iter = token_vec.iter();
    let (_, json_object) = parse_tokens(token_iter);

    return json_object;
}

fn get_key(next_val: &Option<&Token>) -> Option<String> {
    if let Some(next_key) = next_val {
        let key = next_key.get_token();
        if let JsonTokenType::String(val) = key {
            return Some(strip_quotes(val));
        } else {
            panic!("Expected key to be string, instead got: {:?}", key)
        }
    } else {
        panic!("Unexpected end of object");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn get_key_with_none() {
        let next_val = None;
        get_key(&next_val);
    }

    #[test]
    fn get_key_with_string() {
        let key = String::from("String");
        let token = Token::new(JsonTokenType::String(key.clone()), key.clone());
        let next_val = Some(&token);

        let ret = get_key(&next_val);
        assert_eq!(ret.is_some(), true);
        assert_eq!(ret.unwrap(), key);
    }

    #[test]
    #[should_panic]
    fn get_key_with_not_string() {
        let token = Token::new(JsonTokenType::Delimiter(Delimiters::Comma), ",".to_string());
        let next_val = Some(&token);

        get_key(&next_val);
    }

    #[test]
    fn remove_first_and_last_brace_with_empty_vec() {
        let token_vec = vec![];
        let res = remove_first_and_last_brace(token_vec);
        assert!(res.len() == 0)
    }

    #[test]
    #[should_panic(
        expected = "Object does not start with left brace, instead I got: Token { token: Boolean(true), raw: \"true\" }"
    )]
    fn remove_first_and_last_brace_with_no_braces() {
        let json_token = JsonTokenType::Boolean(true);
        let token_vec = vec![Token::new(json_token, "true".to_string())];
        remove_first_and_last_brace(token_vec);
    }

    #[test]
    #[should_panic(
        expected = "Object does not end with right brace, instead I got: Token { token: Boolean(true), raw: \"true\" }"
    )]
    fn remove_first_and_last_brace_with_no_last_brace() {
        let start_token = Token::new(
            JsonTokenType::Delimiter(Delimiters::LeftBrace),
            "{".to_string(),
        );
        let end_token = Token::new(JsonTokenType::Boolean(true), "true".to_string());
        let token_vec = vec![start_token, end_token];
        println!("{:?}", token_vec);
        remove_first_and_last_brace(token_vec);
    }
}
