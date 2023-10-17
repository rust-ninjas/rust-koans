pub fn merge_sort(numbers: Vec<i32>) -> Vec<i32> {
    if numbers.len() <= 1 {
        return numbers;
    }

    let center = (numbers.len() as f64 / 2 as f64).floor() as usize;

    let (left, right) = numbers.split_at(center);

    return merge(merge_sort(left.to_vec()), merge_sort(right.to_vec()));
}

fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![];

    while !left.is_empty() && !right.is_empty() {
        if left[0] < right[0] {
            results.push(left.remove(0));
        } else {
            results.push(right.remove(0));
        }
    }

    results.extend(left);
    results.extend(right);

    return results;
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
