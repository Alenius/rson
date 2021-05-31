pub fn lex(json_as_string: String) -> Vec<String> {
    let mut word_arr: Vec<String> = vec![];
    let mut current_word = String::new();
    let mut open_quote = false;
    let mut open_digit = false;

    // TODO: Reimplement this as iterator
    for json_char in json_as_string.chars() {
        match json_char {
            '\n' | ' ' => (),
            '\"' => {
                if open_quote {
                    current_word.push('\"');
                    word_arr.push(current_word);
                    current_word = String::new();
                    open_quote = false;
                } else {
                    // opening the word for writing
                    open_quote = true;
                    // save the quote for the string, makes it easier
                    // to identify as string
                    current_word.push('\"')
                }
            }
            // if the string is open, everything is part of that string so just push it
            new_char if open_quote => {
                current_word.push(new_char);
            }
            // check if number or start of a negative number
            digit if json_char.is_numeric() || json_char == '-' => {
                if !open_digit {
                    open_digit = true;
                }
                current_word.push(digit);
            }
            // TODO: this can probably be done better
            // since strings must be inside quotes, letters here must be part of boolean
            bool_char if json_char.is_ascii_alphabetic() => {
                current_word.push(bool_char);
                if current_word == "true" || current_word == "false" {
                    word_arr.push(current_word);
                    current_word = String::new();
                }
            }
            // separator is either , ] in array or } if it's last in object depending on position within the array
            separator if open_digit => {
                open_digit = false;
                word_arr.push(current_word);
                current_word = String::new();
                word_arr.push(separator.to_string());
            }
            _ => {
                // rest of the stuff, parentesis, brackets etc
                word_arr.push(json_char.to_string());
            }
        }
    }

    return word_arr;
}

