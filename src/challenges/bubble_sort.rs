/// # Coding Exercise: Bubble Sort Algorithm
///
/// **Objective:** Implement the bubble sort algorithm to sort an input array in ascending order.
///
/// Bubble sort is a simple sorting algorithm that works by repeatedly stepping through the list, comparing each pair of adjacent items and swapping them if they are in the wrong order. Over several iterations, larger values move to the end (they 'bubble up'), while smaller values settle at the start.
///
/// ## Example
///
/// ```rust
/// bubble_sort(vec![100, -40, 500, -124, 0, 21, 7]);
/// // Should return: vec![-124, -40, 0, 7, 21, 100, 500];
/// ```
///
/// **Task:** Implement the `bubble_sort` function as described above.
///
/// ## See also
///
/// [Wikipedia: Bubble Sort](https://en.wikipedia.org/wiki/Bubble_sort)

pub fn bubble_sort(mut numbers: Vec<i32>) -> Vec<i32> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        assert_eq!(
            bubble_sort(vec![100, -40, 500, -124, 0, 21, 7]),
            vec![-124, -40, 0, 7, 21, 100, 500]
        );
    }

    #[test]
    fn test_bubble_sort_empty() {
        assert_eq!(bubble_sort(vec![]), vec![]);
    }

    #[test]
    fn test_bubble_sort_single_element() {
        assert_eq!(bubble_sort(vec![5]), vec![5]);
    }

    #[test]
    fn test_bubble_sort_already_sorted() {
        assert_eq!(bubble_sort(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_reverse_sorted() {
        assert_eq!(bubble_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_duplicates() {
        assert_eq!(bubble_sort(vec![5, 2, 3, 5, 2, 1]), vec![1, 2, 2, 3, 5, 5]);
    }
}
