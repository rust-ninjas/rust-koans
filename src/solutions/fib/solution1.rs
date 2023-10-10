fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return fib(n - 2) + fib(n - 1);
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
