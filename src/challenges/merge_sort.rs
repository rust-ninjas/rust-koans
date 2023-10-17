/// Merge Sort Algorithm
///
/// This function sorts an input array in ascending order using the merge sort algorithm. Merge sort is a divide-and-conquer algorithm that works by dividing the unsorted list into n sublists, each containing one element (a list of one element is considered sorted), and then repeatedly merging sublists to produce new sorted sublists until there is only one sublist remaining.
///
/// The task is to implement this function.
///
/// # Example
///
/// ```
/// merge_sort(vec![100, -40, 500, -124, 0, 21, 7]);
/// // Returns: vec![-124, -40, 0, 7, 21, 100, 500];
/// ```
pub fn merge_sort(numbers: Vec<i32>) -> Vec<i32> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn test_merge_sort() {
        assert_eq!(
            merge_sort(vec![100, -40, 500, -124, 0, 21, 7]),
            vec![-124, -40, 0, 7, 21, 100, 500]
        );
    }

    #[test]
    fn test_merge_sort_empty() {
        assert_eq!(merge_sort(vec![]), vec![]);
    }

    #[test]
    fn test_merge_sort_single_element() {
        assert_eq!(merge_sort(vec![5]), vec![5]);
    }

    #[test]
    fn test_merge_sort_already_sorted() {
        assert_eq!(merge_sort(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse_sorted() {
        assert_eq!(merge_sort(vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_duplicates() {
        assert_eq!(merge_sort(vec![5, 2, 3, 5, 2, 1]), vec![1, 2, 2, 3, 5, 5]);
    }
}
