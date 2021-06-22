use rson::{parse, parse_ann};
use std::fs;

fn main() {
    let json_content = fs::read_to_string("./test_files/1.json")
        .expect("Something went wrong when reading the file");

        println!("json {}", json_content);

    parse_ann(json_content);
    // json_object.print();
    // println!("{}", json_object.get_value("object").unwrap())
}
