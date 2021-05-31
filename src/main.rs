use std::fs;

mod lexer;
mod parser;
mod utils;
mod types;

fn main() {
    let json_content = fs::read_to_string("./test_files/1.json")
        .expect("Something went wrong when reading the file");

    let tokens = lexer::lex(json_content);
    let json_object = parser::parse(tokens);
    // println!("{:?}", json_object)
    json_object.print();
}
