pub fn anagrams(s1: &str, s2: &str) -> bool {
    let mut first = s1
        .to_lowercase()
        .chars()
        .filter(|char| char.is_alphanumeric())
        .collect::<Vec<char>>();
    first.sort();

    let mut second = s2
        .to_lowercase()
        .chars()
        .filter(|char| char.is_alphanumeric())
        .collect::<Vec<char>>();
    second.sort();

    first.eq(&second)
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
