pub fn capitalize(input_string: &str) -> String {
    input_string
        .chars()
        .fold(String::new(), |mut output_string, char| {
            if output_string.is_empty()
                || output_string
                    .chars()
                    .last()
                    .map(|c| c.is_whitespace())
                    .unwrap_or(false)
            {
                output_string.push(char.to_ascii_uppercase());
            } else {
                output_string.push(char);
            }

            return output_string;
        })
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
