use std::env;
use std::fs;

fn main() {
    let json_content = fs::read_to_string("./test_files/1.json")
        .expect("Something went wrong when reading the file");

    let mut char_arr: Vec<char> = vec![];

    // remove whitespace, newlines, quotes
    for json_char in json_content.chars() {
        match json_char {
            ' ' | '\n' | '\"' => (),
            _ => char_arr.push(json_char),
        }
    }

    let mut token_arr: Vec<String> = vec![String::new()];
    for ch in char_arr {
        match ch {
            ':' => token_arr.push(String::new()), // push empty str when splitting entry
            _ => {
                let last_elem = token_arr.last_mut().unwrap();
                last_elem.push(ch)
            }
        }
    }

    println!("{:?}", token_arr);
}
