use super::types::JsonValue;

// a string will guaranteed start and end with quotes
pub fn strip_quotes(string: String) -> String {
    let mut stripped_string = string.clone();
    // there is probably a more idiomatic way of doing this?
    if is_string_token(&string) {
        stripped_string.pop(); // remove last
        stripped_string.remove(0); // remove first char

        return stripped_string;
    } else {
        panic!(
            "Cannot remove quotes on something that's not a proper string: {}",
            string
        );
    }
}

pub fn from_str_to_bool(token: String) -> bool {
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

pub fn typify_token(token: String) -> JsonValue {
    let first_char = token.chars().next().unwrap();

    // check if string
    if first_char == '\"' {
        let stripped_token = strip_quotes(token);
        return JsonValue::String(stripped_token);
    }

    // check if number
    if token.as_str().parse::<f64>().is_ok() {
        let number = token.as_str().parse::<f64>().unwrap();
        return JsonValue::Num(number);
    }

    match token.as_str() {
        "true" | "false" => {
            let boolean = from_str_to_bool(token);
            return JsonValue::Bool(boolean);
        }
        _ => panic!("Unknown token type, first_char: {}", first_char),
    }
}


// check that a string is a string by starting and ending with quotes
pub fn is_string_token(string: &String) -> bool {
    let start_ok = string.as_str().starts_with("\"");
    let end_ok = string.as_str().ends_with("\"");
    return start_ok && end_ok;
}