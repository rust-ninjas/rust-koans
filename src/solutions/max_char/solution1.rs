use std::collections::HashMap;

pub fn max_char(input: &str) -> Option<char> {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    let mut output_char_option: Option<char> = None;

    input.chars().for_each(|char| {
        *char_map.entry(char).or_insert(0) += 1;

        if let Some(output_char) = output_char_option {
            if char_map.get(&char) > char_map.get(&output_char) {
                output_char_option = Some(char);
            }
        } else {
            output_char_option = Some(char);
        }
    });
    output_char_option
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
