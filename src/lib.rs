use types::JsonObject;

mod lexer;
mod parser;
pub mod types;

pub fn parse(json_content: String) -> JsonObject {
    let tokens = lexer::lex(json_content);
    let json_object = parser::parse(tokens);

    return json_object
}