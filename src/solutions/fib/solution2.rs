fn fib(n: u32) -> u32 {
    let mut last_pair: (u32, u32) = (0, 1);

    if n == 0 {
        last_pair.0
    } else if n == 1 {
        last_pair.1
    } else {
        for _ in 2..=n {
            let new_prev = last_pair.1;
            let new_last = last_pair.0 + last_pair.1;

            last_pair = (new_prev, new_last);
        }

        last_pair.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(4), 3);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(0), 0);
        assert_eq!(fib(10), 55);
    }
}
