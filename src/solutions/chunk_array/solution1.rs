pub fn chunk_array<T: Clone>(input: &[T], chunk_array_size: usize) -> Vec<Vec<T>> {
    input
        .chunks(chunk_array_size)
        .map(|chunk_array| chunk_array.to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_array() {
        assert_eq!(chunk_array(&[1, 2, 3, 4], 2), vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(
            chunk_array(&[1, 2, 3, 4, 5], 2),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
        assert_eq!(
            chunk_array(&[1, 2, 3, 4, 5, 6, 7, 8], 3),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]]
        );
        assert_eq!(
            chunk_array(&[1, 2, 3, 4, 5], 4),
            vec![vec![1, 2, 3, 4], vec![5]]
        );
        assert_eq!(chunk_array(&[1, 2, 3, 4, 5], 10), vec![vec![1, 2, 3, 4, 5]]);
    }
}
