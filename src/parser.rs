use std::slice::Iter;
use std::vec;
use super::utils::{typify_token, strip_quotes, is_string_token};
use super::types::{JsonValue, JsonObject};



fn parse_vec(iter: Iter<String>) -> (JsonValue, Iter<String>) {
    let mut iter = iter.clone();
    let mut arr: Vec<JsonValue> = vec![];
    // TODO: break this out to handle nested arrays
    while let Some(elem) = iter.next() {
        if elem.eq("]") {
            break;
        }
        if elem.eq(",") {
            continue;
        }
        // nested vec
        if elem.eq("[") {
            let (parsed_vec, return_iter) = parse_vec(iter);
            iter = return_iter;
            arr.push(parsed_vec);
            continue;
        }

        let typed_elem = typify_token(elem.to_owned());
        arr.push(typed_elem);
    }

    return (JsonValue::Vec(arr), iter);
}


// returning an iter makes it easy to handle recursive case with nested objects 
fn parse_tokens(token_iter: Iter<String>) -> (JsonObject, Iter<String>) {
    let mut token_iter = token_iter;
    let mut json_object = JsonObject::new();

    while let Some(token) = token_iter.next() {
        let key: String;
        // handle keys and things that comes first in the line
        if token.eq(",") {
            // comma separating the lines
            continue;
        } else if token.eq("}") {
            // handling closing of an object.
            return (json_object, token_iter)
        } else {
            // double check that the key value actually is a string
            if !is_string_token(&token) {
                panic!("The key is not a string, it's: {}", token);
            }

            let stripped_token = strip_quotes(token.to_owned());
            key = stripped_token;
        }

        // handle delimiters
        if let Some(delimiter) = token_iter.next() {
            if delimiter.ne(":") {
                panic!("The delimiter is not a colon, it's: {}", delimiter);
            }
        }

        // handle values
        if let Some(token) = token_iter.next() {
            match token.as_str() {
                // object opened
                "{" => {
                    // returning the consumed iter
                    let (object, return_iter) = parse_tokens(token_iter);
                    token_iter = return_iter;
                    json_object.insert(key, JsonValue::Object(object));
                    continue;
                }
                // object closed
                "}" => {
                    return (json_object, token_iter);
                }
                // handle array
                "[" => {
                    let (parsed_arr, iterator) = parse_vec(token_iter);
                    token_iter = iterator; // give the iterator back
                    json_object.insert(key.to_owned(), parsed_arr);
                    continue;
                }
                _ => {
                    // handle numbers, strings and booleans
                    let value = typify_token(token.to_owned());
                    json_object.insert(key.to_owned(), value);
                }
            }
        } else {
            panic!("Created a key, but there is no value. Unbalanced JSON.")
        }
    }

    return (json_object, token_iter);
}

pub fn parse<'a>(lexed_json: Vec<String>) -> JsonObject {
    let mut lexed_json = lexed_json.clone();
    // TODO: check that these are correct.
    lexed_json.pop();
    lexed_json.remove(0);

    let token_iter = lexed_json.iter();
    let (json_object, _) = parse_tokens(token_iter);

    return json_object;
}
