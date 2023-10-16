/// Coding Exercise - Max Chars
///
/// Given a string, return the character that is most commonly used in the string.
///
/// # Examples
///
/// ```
/// let result = max_char("abcccccccd");
/// assert_eq!(result, Some('c'));
///
/// let result = max_char("apple 1231111");
/// assert_eq!(result, Some('1'));
/// ```
pub fn max_char(input: &str) -> Option<char> {
    todo!();
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
