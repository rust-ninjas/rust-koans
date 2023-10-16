use std::collections::HashMap;

pub fn max_char(input: &str) -> Option<char> {
    let mut charMap: HashMap<char, i32> = HashMap::new();
    let mut outputChar: Option<char> = None;

    input.chars().for_each(|char| {
        *charMap.entry(char).or_insert(0) += 1;
    });

    for (key, value) in &charMap {
        match outputChar {
            Some(output_char_key) => {
                charMap.get(&output_char_key).and_then(|output_char_value| {
                    if value > output_char_value {
                        outputChar = Some(key.clone());
                    };
                    Some(key)
                });
            }
            None => outputChar = Some(key.clone()),
        }
    }

    return outputChar;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_char() {
        assert_eq!(max_char("abcccccccd"), Some('c'));
        assert_eq!(max_char("apple 1231111"), Some('1'));
        assert_eq!(max_char("Hello World!"), Some('l'));
        assert_eq!(max_char("Rust Programming Language"), Some('g'));
        assert_eq!(max_char("AABBBCC"), Some('B'));
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(max_char(""), None);
    }

    #[test]
    fn test_single_char_string() {
        assert_eq!(max_char("a"), Some('a'));
    }

    #[test]
    fn test_all_unique_chars() {
        assert_eq!(max_char("aabcdef"), Some('a'));
    }

    #[test]
    fn test_string_with_spaces() {
        assert_eq!(max_char("a b c d e f"), Some(' '));
    }

    #[test]
    fn test_string_with_special_chars() {
        assert_eq!(max_char("!@#$%^&*()!!"), Some('!'));
    }
}
