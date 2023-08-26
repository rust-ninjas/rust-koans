/// # Exercise: Array Chunk
///
/// In this exercise, you are tasked to implement a function in Rust that divides a given array into multiple subarrays, where each subarray is of a specified size.
///
/// The function will take two parameters:
///
/// 1. `input`: A slice of items of any type that implements the `Clone` trait.
/// 2. `chunk_size`: An integer that specifies the size of the subarrays.
///
/// The function will return a `Vec<Vec<T>>`, where `T` is the type of the items in the input array.
///
/// Your task is to divide the input array into as many chunks as possible, each of size `chunk_size`. If the number of elements in the input array is not a multiple of `chunk_size`, the last chunk will contain the remaining elements.
///
/// Your function should clone the elements from the input array into the chunks.
///
/// ## Examples:
///
/// - If `chunk([1, 2, 3, 4], 2)` is called, the function should return `[[1, 2], [3, 4]]`.
/// - If `chunk([1, 2, 3, 4, 5], 2)` is called, the function should return `[[1, 2], [3, 4], [5]]`.
/// - If `chunk([1, 2, 3, 4, 5, 6, 7, 8], 3)` is called, the function should return `[[1, 2, 3], [4, 5, 6], [7, 8]]`.
/// - If `chunk([1, 2, 3, 4, 5], 4)` is called, the function should return `[[1, 2, 3, 4], [5]]`.
/// - If `chunk([1, 2, 3, 4, 5], 10)` is called, the function should return `[[1, 2, 3, 4, 5]]`.
///
/// These are the test cases that your implementation will be validated against.
///
/// # Exercise: Array Chunk
///
/// In this exercise, you are tasked to implement a function in Rust that divides a given array into multiple subarrays, where each subarray is of a specified size.
///
/// The function will take two parameters:
///
/// 1. `input`: A slice of items of any type that implements the `Clone` trait.
/// 2. `chunk_size`: An integer that specifies the size of the subarrays.
///
/// The function will return a `Vec<Vec<T>>`, where `T` is the type of the items in the input array.
///
/// Your task is to divide the input array into as many chunks as possible, each of size `chunk_size`. If the number of elements in the input array is not a multiple of `chunk_size`, the last chunk will contain the remaining elements.
///
/// Your function should clone the elements from the input array into the chunks.
///
/// ## Examples:
///
/// - If `chunk([1, 2, 3, 4], 2)` is called, the function should return `[[1, 2], [3, 4]]`.
/// - If `chunk([1, 2, 3, 4, 5], 2)` is called, the function should return `[[1, 2], [3, 4], [5]]`.
/// - If `chunk([1, 2, 3, 4, 5, 6, 7, 8], 3)` is called, the function should return `[[1, 2, 3], [4, 5, 6], [7, 8]]`.
/// - If `chunk([1, 2, 3, 4, 5], 4)` is called, the function should return `[[1, 2, 3, 4], [5]]`.
/// - If `chunk([1, 2, 3, 4, 5], 10)` is called, the function should return `[[1, 2, 3, 4, 5]]`.
///
/// These are the test cases that your implementation will be validated against.
///
/// Remember, the function needs to be generic and work for not only integers but any type that implements the `Clone` trait.Remember, the function needs to be generic and work for not only integers but any type that implements the `Clone` trait.
pub fn chunk_array<T: Clone>(input: &[T], chunk_array_size: usize) -> Vec<Vec<T>> {
    todo!();
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
