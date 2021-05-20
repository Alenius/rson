use std::collections::HashMap;
use std::{fs, vec};

// TODO: maybe redo implementation and save quotes?
// this makes it easier to check if the value is string or something else
// in the parser
fn lexer(json_as_string: String) -> Vec<String> {
    let mut word_arr: Vec<String> = vec![];
    let mut current_word = String::new();
    let mut open_quote = false;
    let mut open_digit = false;

    // TODO: handle booleans
    for json_char in json_as_string.chars() {
        match json_char {
            '\n' | ' ' => (),
            '\"' => {
                if open_quote {
                    current_word.push('\"');
                    word_arr.push(current_word);
                    current_word = String::new();
                    open_quote = false;
                } else {
                    // opening the word for writing
                    open_quote = true;
                    // save the quote for the string, makes it easier
                    // to identify as string
                    current_word.push('\"')
                }
            }
            // if the string is open, everything is part of that string so just push it
            new_char if open_quote => {
                current_word.push(new_char);
            }
            digit if json_char.is_numeric() => {
                if !open_digit {
                    open_digit = true;
                }
                current_word.push(digit);
            }
            // TODO: this can probably be done better
            // since strings must be inside quotes, letters here must be part of boolean
            bool_char if json_char.is_ascii_alphabetic() => {
                current_word.push(bool_char);
                if current_word == "true" || current_word == "false" {
                    word_arr.push(current_word);
                    current_word = String::new();
                }
            }
            // separator is either , ] in array or } if it's last in object depending on position within the array
            separator if open_digit => {
                open_digit = false;
                word_arr.push(current_word);
                current_word = String::new();
                word_arr.push(separator.to_string());
            }
            _ => {
                // rest of the stuff, parentesis, brackets etc
                word_arr.push(json_char.to_string());
            }
        }
    }

    return word_arr;
}

#[allow(dead_code)]
#[derive(Debug)]
enum JsonValue<'a> {
    String(String),
    Num(u32),
    Vec(Vec<JsonValue<'a>>), // ok so this works! self referencing
    Bool(bool),
    Object(HashMap<&'a String, u32>),
}

fn check_if_string_is_numeric(string: &str) -> bool {
    for char in string.chars() {
        if !char.is_numeric() {
            return false;
        }
    }
    return true;
}

// a string will guaranteed start and end with quotes
fn strip_quotes(string: String) -> String {
    let mut stripped_string = string.clone();
    println!("STRIPPED {:?}", string);
    stripped_string.pop(); // remove last
    stripped_string.remove(0); // remove first char

    return stripped_string;
}

fn from_str_to_bool(token: String) -> bool {
    let boolean: bool;
    if token == "true" {
        boolean = true;
    } else if token == "false" {
        boolean = false;
    } else {
        panic!("Boolean is not true or false");
    }
    return boolean;
}

fn typify_token(token: String) -> JsonValue<'static> {
    let first_char = token.chars().next().unwrap();

    // how to handle arrays and objects?
    match first_char {
        '\"' => {
            let stripped_token = strip_quotes(token);
            return JsonValue::String(stripped_token);
        }
        _ if first_char.is_alphabetic() => {
            let boolean = from_str_to_bool(token);
            return JsonValue::Bool(boolean);
        }
        _ if first_char.is_numeric() => {
            // does this work for negative numbers?
            return JsonValue::Num(first_char.to_digit(10).unwrap());
        }
        _ => panic!("Unknown token type"),
    }
}

fn parser<'a>(lexed_json: Vec<String>) -> HashMap<String, JsonValue<'a>> {
    let mut json_object: HashMap<String, JsonValue> = HashMap::new();

    let mut current_key: String = String::new();
    let mut current_value: JsonValue;
    let mut current_arr: Vec<JsonValue>;
    let mut is_array: bool = false;

    for token in lexed_json {
        println!("{:?}", token);
        match token.as_str() {
            "{" | ":" | "," => (),
            _ if current_key.is_empty() => {
                let stripped_token = strip_quotes(token);
                current_key = stripped_token;
            }
            "[" => {
                is_array = true;
                current_arr = vec![];
            }
            "]" if is_array => {
                is_array = false;
                json_object.insert(current_key, JsonValue::Vec(current_arr));
                current_arr = vec![];
            }
            // it's a string, just pushing the entire thing should be fine
            _ if token.chars().next().unwrap() == '\"' => {
                current_value = JsonValue::String(String::from(&token.clone()));
                println!("{:?}", current_value);
                json_object.insert(current_key, current_value);
                current_key = String::new();
                // possible to reinitialize the current_value?
            }
            elem if is_array => {
                current_arr.push(token);
            }
            _ => {
                ();
            }
        }
    }

    return json_object;
}

fn main() {
    let json_content = fs::read_to_string("./test_files/2.json")
        .expect("Something went wrong when reading the file");

    let tokens = lexer(json_content);
    // println!("{:?}", tokens);
    let json_object = parser(tokens);
    println!("{:?}", json_object)
}
