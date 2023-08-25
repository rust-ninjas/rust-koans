pub fn capitalize(input_string: &str) -> String {
    let mut ready_for_upper = true;

    input_string
        .chars()
        .map(|current_char| {
            let mut output_char = current_char;

            if ready_for_upper {
                output_char = current_char.to_uppercase().next().unwrap();
            }

            if current_char == ' ' {
                ready_for_upper = true;
            } else {
                ready_for_upper = false;
            }

            output_char
        })
        .collect()
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
