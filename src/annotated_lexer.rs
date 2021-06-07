#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Num(f64),
    Vec(Vec<JsonValue>), // ok so this works! self referencing
    Bool(bool),
    Delimiter(char),
    Whitespace(char)
}

#[derive(Debug)]
enum TokenType {
    Delimiter,
    Key,
    Whitespace,
    String,
    Number,
    Boolean,
    Array,
    Object,
}


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: JsonValue,
    raw: String
}

impl Token {
    fn new(token_type: TokenType, value: JsonValue, raw: String) -> Token {
        return Token {
            token_type,
            value,
            raw
        }
    }    
}


pub fn annotated_lexer(json: String) -> Vec<Token> {
    let token: Token = Token {
        token_type: TokenType::Delimiter,
        value: JsonValue::Delimiter(','),
        raw: ",".to_owned(),
    };

    let mut iter = json.chars();
    let mut lookahead: Option<char> = iter.next();
    let mut token_vec: Vec<Token> = vec![];
    loop {
        match lookahead {
            None => break,
            Some(val) => {
                match val {
                  '{' | '}' | '['| ']' | ',' | ':'  => {
                    token_vec.push(Token::new(TokenType::Delimiter, JsonValue::Delimiter(val), val.to_string()))
                  }
                  '\n' | ' ' => {
                      token_vec.push(Token::new(TokenType::Whitespace, JsonValue::Whitespace(val), val.to_string()))
                  }
                  // string
                  '\"' => {
                    let mut string_builder = String::new();
                    string_builder.push(val); // push first quote
                    loop {
                        let val = iter.next();
                        if val.is_none() {
                            panic!("Unexpected end of input, string not complete")
                        } else {
                            let val = val.unwrap();
                            string_builder.push(val);
                            if val == '\"' && !string_builder.ends_with("\\\"") {
                                let finished_string = string_builder.to_string();
                                token_vec.push(Token::new(TokenType::String, JsonValue::String(finished_string.clone()), finished_string));
                                break;
                            }
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