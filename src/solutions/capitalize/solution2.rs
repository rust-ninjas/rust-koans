fn first_char_to_upper_case(input_string: &str) -> String {
    let mut chars = input_string.chars();

    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn capitalize(input_string: &str) -> String {
    input_string
        .split(' ')
        .map(|word| first_char_to_upper_case(word))
        .collect::<Vec<String>>()
        .join(" ")
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
