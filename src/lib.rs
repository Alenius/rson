use types::JsonObject;

mod lexer;
mod parser;
pub mod types;

pub fn parse(json_content: String) -> JsonObject {
    let tokens = lexer::lex(json_content);
    let json_object = parser::parse(tokens);

    return json_object;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string_values() {
        let str = "{
            \"string\": \"I am a string\",
            \"str_with_num\": \"I am a string w1th numb3rs\"
        }";
        parse(str.to_owned());
    }

    #[test]
    fn parse_number_values() {
        let str = "{
            \"number\": 1337,
            \"float\": \"13.37\"
        }";
        parse(str.to_owned());
    }

    #[test]
    fn parse_vec() {
        let str = "{
            \"array\": [1, 2, 3]
        }";
        parse(str.to_owned());
    } 

    #[test]
    fn parse_nested_vec() {
        let str = "{
            \"array\": [[1, 2], [3, 4]]
        }";
        parse(str.to_owned());
    }

    #[test]
    fn parse_object_in_vec() {
        let str = "{
            \"array\": [{\"key\": 1}, [3, 4]]
        }";
        parse(str.to_owned());
    }

    #[test]
    fn parse_object() {
        let str = "{
            \"object\": {\"key\": \"string value\"}
        }";
        parse(str.to_owned());
    }

    #[test]
    fn parse_nested_object() {
        let str = "{
            \"object\": {\"key\": {\"nested_key\": \"string value\"}}
        }";
        parse(str.to_owned());
    }

    #[test]
    #[should_panic(expected="Unexpected end of object")]
    fn panic_on_obj_ending_with_comma() {
        let str = "{
           \"key\": \"ending in comma\",
        }";
        parse(str.to_owned());
    }

    #[test]
    #[should_panic(expected="Lexer doesn't understand value: =")]
    fn panic_on_wrong_key_value_delimiter() {
        let str = "{
           \"key\"= \"value\",
        }";
        parse(str.to_owned());
    }

    #[test]
    #[should_panic]
    fn panic_on_comma_decimal_sign() {
        let str = "{
            \"num_with_comma\": 13,37
        }";
        parse(str.to_owned());
    }
}
