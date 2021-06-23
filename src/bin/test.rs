use rson::{parse};
use std::fs;

fn main() {
    let json_content = fs::read_to_string("./test_files/test_package.json")
        .expect("Something went wrong when reading the file");

    let obj = parse(json_content);
    println!("Res: {:?}", obj);
}
