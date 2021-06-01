use std::fs;

mod lexer;
mod parser;
mod utils;
mod types;

fn main() {
    let json_content = fs::read_to_string("./test_files/1.json")
        .expect("Something went wrong when reading the file");

    let tokens = lexer::lex(json_content);
    let mut json_object = parser::parse(tokens);
    // println!("{:?}", json_object)
    json_object.print();
    json_object.delete_key("strong");
    println!("----");
    json_object.print();
}
