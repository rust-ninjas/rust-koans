/// # Coding Challenge: Validating Anagrams
/// Determines whether two strings are anagrams of each other.
///
/// An anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.
///
/// ## Arguments
///
/// * `s1` - A string to compare.
/// * `s2` - Another string to compare.
///
/// ## Returns
///
/// `true` if the two strings are anagrams of each other, `false` otherwise.
///
/// ## Examples
///
/// ```
/// assert_eq!(anagrams("rail safety", "fairy tales"), true);
/// assert_eq!(anagrams("RAIL! SAFETY!", "fairy tales"), true);
/// assert_eq!(anagrams("Hi there", "Bye there"), false);
/// ```
///
/// ## Notes
///
/// This function ignores non-alphanumeric characters and is case-insensitive.
///
/// ## See also
///
/// * [Wikipedia article on anagrams](https://en.wikipedia.org/wiki/Anagram)
pub fn anagrams(s1: &str, s2: &str) -> bool {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagrams() {
        assert_eq!(anagrams("rail safety", "fairy tales"), true);
        assert_eq!(anagrams("RAIL! SAFETY!", "fairy tales"), true);
        assert_eq!(anagrams("Hi there", "Bye there"), false);
        assert_eq!(anagrams("Dormitory", "dirty room"), true);
        assert_eq!(anagrams("listen", "silent"), true);
        assert_eq!(anagrams("Astronomer", "Moon starer"), true);
        assert_eq!(anagrams("The eyes", "They see"), true);
        assert_eq!(anagrams("George Bush", "He bugs Gore"), true);
        assert_eq!(anagrams("I am Lord Voldemort", "Tom Marvolo Riddle"), true);
        assert_eq!(anagrams("Madam Curie", "Radium came"), true);
        assert_eq!(anagrams("Eleven plus two", "Twelve plus one"), true);
        assert_eq!(
            anagrams(
                "One good turn deserves another",
                "Do rogues endorse that? No, never!"
            ),
            true
        );
    }
}
