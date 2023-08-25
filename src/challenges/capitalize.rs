/// # Exercise: Capitalize
///
/// Given a string, capitalize the first letter of each word in the string.
///
/// ## Examples
///
/// ```
/// assert_eq!(capitalize("hello world"), "Hello World");
/// assert_eq!(capitalize("rust programming language"), "Rust Programming Language");
/// assert_eq!(capitalize("sunflowers grow tall"), "Sunflowers Grow Tall");
/// assert_eq!(capitalize("oh no, it's raining again!"), "Oh No, It's Raining Again!");
/// ```
///
/// ## Arguments
///
/// * `input_string` - A string slice that holds the input string.
///
/// ## Returns
///
/// A string slice that holds the capitalized string.
pub fn capitalize(input_string: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello world"), "Hello World");
        assert_eq!(
            capitalize("rust programming language"),
            "Rust Programming Language"
        );
        assert_eq!(capitalize("sunflowers grow tall"), "Sunflowers Grow Tall");
        assert_eq!(
            capitalize("oh no, it's raining again!"),
            "Oh No, It's Raining Again!"
        );
    }
}
