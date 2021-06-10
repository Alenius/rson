use core::{panic};

#[derive(Debug)]
enum Delimiters  {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
}

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Num(f64),
    Vec(Vec<JsonValue>), // ok so this works! self referencing
    Bool(bool),
    Whitespace(char)
}

#[derive(Debug)]
enum TokenType {
    Delimiter(Delimiters),
    Whitespace,
    String,
    Number,
    Boolean,
}


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    raw: String
}

impl Token {
    fn new(token_type: TokenType, raw: String) -> Token {
        return Token {
            token_type,
            raw
        }
    }    
}


pub fn annotated_lexer(json: String) -> Vec<Token> {
    let mut iter = json.chars();
    let mut lookahead: Option<char> = iter.next();
    let mut token_vec: Vec<Token> = vec![];
    loop {
        match lookahead {
            None => break,
            Some(val) => {
                match val {
                  '{' => token_vec.push(Token::new(TokenType::Delimiter(Delimiters::LeftBrace), '{'.to_string())),
                  '}' => token_vec.push(Token::new(TokenType::Delimiter(Delimiters::RightBrace), '}'.to_string())),
                  '[' => token_vec.push(Token::new(TokenType::Delimiter(Delimiters::LeftBracket), '['.to_string())),
                  ']' => token_vec.push(Token::new(TokenType::Delimiter(Delimiters::RightBracket), ']'.to_string())),
                  ':' => token_vec.push(Token::new(TokenType::Delimiter(Delimiters::Colon), ':'.to_string())),
                  ',' => token_vec.push(Token::new(TokenType::Delimiter(Delimiters::Comma), ','.to_string())),
                  '\n' | ' ' => {} // ignore whitespace
                  // string
                  '\"' => {
                    let mut string_builder = String::new();
                    string_builder.push(val); // push first quote
                    loop {
                        let next_val = iter.next();
                        if let Some(val) = next_val {
                            string_builder.push(val);

                            let is_quote = val == '\"';
                            let is_escaped_quote = string_builder.ends_with("\\\"") ;
                            if is_quote && !is_escaped_quote {
                                let finished_string = string_builder.to_string();
                                token_vec.push(Token::new(TokenType::String, finished_string));
                                break;
                            }
                        } else {
                            panic!("Unexpected end of input, string not complete")
                        }
                        
                    }
                  }
                  // boolean and null
                  't' | 'f' | 'n' => {
                    let mut bool_builder = String::new();
                    bool_builder.push(val);

                    loop {
                        let next_val = iter.next();
                        if let Some(value) = next_val {
                            if !value.is_alphabetic() {
                                let finished_bool_or_null = bool_builder.to_string();  
                                let bool_token = Token::new(TokenType::Boolean, finished_bool_or_null);
                                token_vec.push(bool_token);
                                break
                            }
                            bool_builder.push(val)
                        } else {
                            panic!("Unexpected end of input, incomplete boolean or null")
                        }
                    }
                  }
                  // numbers
                  num if val.is_numeric() => {
                      let mut num_builder = String::new();
                      num_builder.push(num);

                        loop {
                            let next_val = iter.next();
                            if let Some(value) = next_val {
                                let is_dot = value.eq(&'.');
                                let is_comma = value.eq(&',');

                                // TODO: this doesn't work for hex numbers like 0xx0
                                if !value.is_numeric() && !is_comma && !is_dot  {
                                    let finished_number = num_builder.to_string();                                    
                                    token_vec.push(Token::new(TokenType::Number, finished_number));
                                    break;
                                }
                                num_builder.push(val)
                            } else {
                                panic!("Unexpected end of input, incomplete boolean or null")
                            }
                        }
                  }
                _ => {
                    panic!("Not implemented value, val: {}", val)
                }
                }
            }
        }

        lookahead = iter.next(); 
    }

    return token_vec
}