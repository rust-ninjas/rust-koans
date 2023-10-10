/// # Fibonacci Sequence
///
/// The Fibonacci sequence is a series of numbers in which each number is the sum of the two preceding numbers.
///
/// Write a function `fib` that takes a non-negative integer `n` as input, and returns the `n`-th number in the Fibonacci sequence.
///
/// ## Example
///
/// ```
/// assert_eq!(fib(0), 0);
/// assert_eq!(fib(1), 1);
/// assert_eq!(fib(2), 1);
/// assert_eq!(fib(3), 2);
/// assert_eq!(fib(4), 3);
/// assert_eq!(fib(5), 5);
/// ```
fn fib(n: u32) -> u32 {
    todo!()
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
