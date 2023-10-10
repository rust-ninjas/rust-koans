use crate::challenges::linked_list::{LinkedList, Node};

/// # Task Description
/// Write a function `from_last` that takes a linked list and an integer `n` as input,
/// and returns the data from the node that is `n` steps from the last node in the list.
///
/// - Do not use the 'size' method of the linked list.
/// - If `n` is more than the length of the list return `None`.
///
/// ## Required
/// Solve link_list exercise first and use the LinkedList/Node from it here.
///
/// ## Example
/// ```
/// let mut list = LinkedList::new();
/// list.insert_last('a');
/// list.insert_last('b');
/// list.insert_last('c');
/// list.insert_last('d');
/// assert_eq!(from_last(&list, 2), Some(&'b'));
/// ```
///
/// # Params
/// - list: The linked list
/// - n: The number of elements from the last node in the list
///
/// # Returns
/// - Some(T): The data at the `n`th node from the last of the list.
/// - None: If the list is empty
fn from_last<T: Clone + PartialEq + std::fmt::Debug>(list: &LinkedList<T>, n: usize) -> Option<&T> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_last() {
        let mut list = LinkedList::new();
        list.insert_last('a');
        list.insert_last('b');
        list.insert_last('c');
        list.insert_last('d');
        list.insert_last('e');
        list.insert_last('f');
        list.insert_last('g');

        assert_eq!(from_last(&list, 0), Some(&'g'));
        assert_eq!(from_last(&list, 1), Some(&'f'));
        assert_eq!(from_last(&list, 2), Some(&'e'));
        assert_eq!(from_last(&list, 3), Some(&'d'));
        assert_eq!(from_last(&list, 4), Some(&'c'));
        assert_eq!(from_last(&list, 5), Some(&'b'));
        assert_eq!(from_last(&list, 6), Some(&'a'));
        assert_eq!(from_last(&list, 7), None);
    }
}
