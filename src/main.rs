use std::fs;
use std::collections::HashMap;

// TODO: maybe redo implementation and save quotes? 
// this makes it easier to check if the value is string or something else
// in the lexer
fn lexer(json_as_string: String) -> Vec<String> {
    let mut word_arr: Vec<String> = vec![];
    let mut current_word = String::new();
    let mut open_quote = false;
    let mut open_digit = false;
    
    // TODO: handle booleans
    for json_char in json_as_string.chars() {
        match json_char {
            '\n' | ' '  => (),
            '\"' => {
                if open_quote {
                    word_arr.push(current_word);
                    current_word = String::new();
                    open_quote = false;
                } else {
                    // opening the word for writing
                    open_quote = true;
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

// only implement arrs with mixed strings, arr and bool for now.
// that means no nested arrays or objects.
#[derive(Debug)]
enum VecValue {
    String(String),
    Num(u32),
    Bool(bool)
}

#[allow(dead_code)]
#[derive(Debug)]
enum JsonValue<'a> {
    String(&'a str),
    Vec(Vec<VecValue>),
    Bool(bool),
    Object(HashMap<&'a String, u32>)
}

fn check_if_string_is_numeric(string: &str) -> bool {
    return false;
}

fn parser<'a>(lexed_json: Vec<String>) -> HashMap<String, JsonValue<'a> > {
    let mut json_object: HashMap<String, JsonValue> = HashMap::new();

    let mut current_key: String = String::new();
    let mut current_value: JsonValue;
    let mut is_array: bool = false;

    for token in lexed_json {
        println!("{:?}", token);
        match token.as_str() {
            "{" | ":" => (),
            _ if current_key.is_empty() => {
                current_key = token;
            }
            "[" => {
                is_array = true
            }
            elem if is_array => {
                let is_numeric = check_if_string_is_numeric(elem);
                if (is_numeric)
                current_value(Vec)
            }
            _ => {
                json_object.insert(token, JsonValue::String("Hej"));
            }
        }
    }

    return json_object
}

fn main() {
    let json_content = fs::read_to_string("./test_files/2.json")
        .expect("Something went wrong when reading the file");

    let tokens = lexer(json_content);
    // println!("{:?}", tokens);
    let json_object = parser(tokens);
    println!("{:?}", json_object)
}
