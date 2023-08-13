use core::num;

pub fn bubble_sort(mut numbers: Vec<i32>) -> Vec<i32> {
    let length = numbers.len();
    for i in 0..length {
        for j in 0..(length - i - 1) {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1)
            }
        }
    }

    return numbers;
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
