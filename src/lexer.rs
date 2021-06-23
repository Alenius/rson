use core::panic;

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiters {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Numbers {
    Float(f64),
    Integer(i64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonTokenType {
    Delimiter(Delimiters),
    String(String),
    Number(Numbers),
    Boolean(bool),
    Null,
}

impl JsonTokenType {
    fn new_left_bracket() -> JsonTokenType {
        JsonTokenType::Delimiter(Delimiters::LeftBracket)
    }

    fn new_right_bracket() -> JsonTokenType {
        JsonTokenType::Delimiter(Delimiters::RightBracket)
    }

    fn new_left_brace() -> JsonTokenType {
        JsonTokenType::Delimiter(Delimiters::LeftBrace)
    }

    fn new_right_brace() -> JsonTokenType {
        JsonTokenType::Delimiter(Delimiters::RightBrace)
    }

    fn new_comma() -> JsonTokenType {
        JsonTokenType::Delimiter(Delimiters::Comma)
    }

    fn new_colon() -> JsonTokenType {
        JsonTokenType::Delimiter(Delimiters::Colon)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    token: JsonTokenType,
    raw: String,
}

impl Token {
    pub fn new(token: JsonTokenType, raw: String) -> Token {
        return Token { token, raw };
    }
    pub fn get_token(&self) -> JsonTokenType {
        return self.token.clone();
    }

}

pub fn lex(json: String) -> Vec<Token> {
    let mut iter = json.chars().peekable();
    let mut lookahead: Option<char> = iter.next();
    let mut token_vec: Vec<Token> = vec![];
    loop {
        match lookahead {
            None => break,
            Some(val) => {
                match val {
                    '{' => {
                        token_vec.push(Token::new(JsonTokenType::new_left_brace(), '{'.to_string()))
                    }
                    '}' => token_vec.push(Token::new(
                        JsonTokenType::new_right_brace(),
                        '}'.to_string(),
                    )),
                    '[' => token_vec.push(Token::new(
                        JsonTokenType::new_left_bracket(),
                        '['.to_string(),
                    )),
                    ']' => token_vec.push(Token::new(
                        JsonTokenType::new_right_bracket(),
                        ']'.to_string(),
                    )),
                    ':' => token_vec.push(Token::new(JsonTokenType::new_colon(), ':'.to_string())),
                    ',' => token_vec.push(Token::new(JsonTokenType::new_comma(), ','.to_string())),
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
                                let is_escaped_quote = string_builder.ends_with("\\\"");
                                if is_quote && !is_escaped_quote {
                                    let finished_string = string_builder.to_string();
                                    token_vec.push(Token::new(
                                        JsonTokenType::String(finished_string.clone()),
                                        finished_string.clone(),
                                    ));
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
                            // peek so we don't consume the following comma
                            let peek_next_val = iter.peek();
                            if let Some(value) = peek_next_val {
                                if value.is_alphabetic() {
                                    let next_val = iter.next().unwrap();
                                    bool_builder.push(next_val)
                                } else {
                                    break;
                                }
                            } else {
                                panic!("Unexpected end of input, incomplete boolean or null")
                            }
                        }
                        let finished_bool_or_null = bool_builder.to_string();
                        if finished_bool_or_null.eq("true") {
                            let bool_token =
                                Token::new(JsonTokenType::Boolean(true), finished_bool_or_null);
                            token_vec.push(bool_token);
                        } else if finished_bool_or_null.eq("false") {
                            let bool_token =
                                Token::new(JsonTokenType::Boolean(false), finished_bool_or_null);
                            token_vec.push(bool_token);
                        } else if finished_bool_or_null.eq("null") {
                            let bool_token = Token::new(JsonTokenType::Null, finished_bool_or_null);
                            token_vec.push(bool_token);
                        } else {
                            panic!("Incorrect token found")
                        }
                    }
                    // numbers
                    num if val.is_numeric() || val.eq(&'-') => {
                        let mut num_builder = String::new();
                        num_builder.push(num);

                        loop {
                            let peeked_next_val = iter.peek();
                            if let Some(value) = peeked_next_val {
                                let is_not_num =
                                    value.eq(&',') || value.eq(&'\n') || value.eq(&']');

                                // TODO: this doesn't work for hex numbers like 0xx0
                                if is_not_num {
                                    let is_float = num_builder.contains(".");
                                    if is_float {
                                        let parsed_float = num_builder.parse::<f64>();
                                        match parsed_float {
                                            Ok(number) => {
                                                token_vec.push(Token::new(
                                                    JsonTokenType::Number(Numbers::Float(number)),
                                                    number.to_string(),
                                                ));
                                                break;
                                            }
                                            Err(e) => {
                                                panic!(
                                                    "Something went wrong when lexing number: {:?}",
                                                    e
                                                )
                                            }
                                        }
                                    } else {
                                        let parsed_int = num_builder.parse::<i64>();
                                        match parsed_int {
                                            Ok(number) => {
                                                token_vec.push(Token::new(
                                                    JsonTokenType::Number(Numbers::Integer(number)),
                                                    number.to_string(),
                                                ));
                                                break;
                                            }
                                            Err(e) => {
                                                panic!(
                                                    "Something went wrong when lexing number: {:?}",
                                                    e
                                                )
                                            }
                                        }
                                    }
                                }

                                num_builder.push(value.to_owned());
                                iter.next();
                            } else {
                                panic!("Unexpected end of input, incomplete num or null")
                            }
                        }
                    }
                    _ => {
                        panic!("Lexer doesn't understand value: {}", val)
                    }
                }
            }
        }

        lookahead = iter.next();
    }

    return token_vec;
}
