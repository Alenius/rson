use std::{ fs, env };
use rson::{types::JsonObject, parse};

fn main() {

    let json_content = fs::read_to_string("./test_files/1.json")
        .expect("Something went wrong when reading the file");

    // println!("{:?}", json_object)
    let mut json_object = parse(json_content);
    json_object.print();
    json_object.delete_key("strong");
    println!("----");
    json_object.print();
}
