/// # Program Design

/// Design a program that generates a sequence of numbers from 1 to n and returns it as a vector of strings. However, there are certain rules for generating the values:

/// - If a number is a multiple of three, replace it with the string "fizz".
/// - If a number is a multiple of five, replace it with the string "buzz".
/// - If a number is a multiple of both three and five, replace it with the string "fizzbuzz".

/// For example, if the input is 5, the program should return the following vector of strings:

/// `["1", "2", "fizz", "4", "buzz"]`

/// ## Task

/// Your task is to write the implementation of the `fizz_buzz` function in Rust, which takes an unsigned integer `n` as input and returns a vector of strings that follows the given rules.
fn fizz_buzz(n: u32) -> Vec<String> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz_5() {
        let result = fizz_buzz(5);
        assert_eq!(result, vec!["1", "2", "fizz", "4", "buzz"]);
    }

    #[test]
    fn test_fizz_buzz_15() {
        let result = fizz_buzz(15);
        assert_eq!(
            result,
            vec![
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz",
                "13", "14", "fizzbuzz"
            ]
        );
    }
}
