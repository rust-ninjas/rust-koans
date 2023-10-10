fn fizz_buzz(n: u32) -> Vec<String> {
    (1..=n)
        .map(|num| {
            if num % 3 == 0 && num % 5 == 0 {
                "fizzbuzz".to_string()
            } else if num % 3 == 0 {
                "fizz".to_string()
            } else if num % 5 == 0 {
                "buzz".to_string()
            } else {
                num.to_string()
            }
        })
        .collect()
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
