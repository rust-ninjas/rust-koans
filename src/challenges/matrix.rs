/// Matrix Spiral
///
/// Given an integer N, write a function that returns an NxN matrix in a spiral pattern.
///
/// # Examples
///
/// ```
/// use rust_koans::challenges::matrix::matrix;
///
/// assert_eq!(matrix(2), vec![
///     vec![1, 2],
///     vec![4, 3]
/// ]);
///
/// assert_eq!(matrix(3), vec![
///     vec![1, 2, 3],
///     vec![8, 9, 4],
///     vec![7, 6, 5]
/// ]);
///
/// assert_eq!(matrix(4), vec![
///     vec![1, 2, 3, 4],
///     vec![12, 13, 14, 5],
///     vec![11, 16, 15, 6],
///     vec![10, 9, 8, 7]
/// ]);
/// ```
pub fn matrix(n: usize) -> Vec<Vec<usize>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_2() {
        assert_eq!(matrix(2), vec![vec![1, 2], vec![4, 3]]);
    }

    #[test]
    fn test_matrix_3() {
        assert_eq!(matrix(3), vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]);
    }

    #[test]
    fn test_matrix_4() {
        assert_eq!(
            matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }

    #[test]
    fn test_matrix_5() {
        assert_eq!(
            matrix(5),
            vec![
                vec![1, 2, 3, 4, 5],
                vec![16, 17, 18, 19, 6],
                vec![15, 24, 25, 20, 7],
                vec![14, 23, 22, 21, 8],
                vec![13, 12, 11, 10, 9]
            ]
        );
    }
}
