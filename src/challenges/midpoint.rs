use crate::challenges::linked_list::LinkedList;
/// # Exercise midpoint
/// Implement a function, `midpoint`, for a linked list that retrieves the 'middle' node.
/// If the list has an even number of elements, return the last node of the first half.
/// - A counter variable should not be used.
/// - The size of the list should not be retrieved.
/// - The list should only be iterated over once.
///
/// ## Required
/// Solve linked_list exercise first and use the LinkedList from it here.
///
/// ## Example
/// ```
/// let mut list = LinkedList::new();
/// list.insert_last('a');
/// list.insert_last('b');
/// list.insert_last('c');
/// assert_eq!(list.midpoint(), Some(&'b'));
/// ```
fn midpoint<T: Clone + PartialEq + Eq>(list: &LinkedList<T>) -> Option<&T> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midpoint() {
        let mut list = LinkedList::new();
        assert_eq!(midpoint(&list), None);

        list.insert_last('a');
        assert_eq!(midpoint(&list), Some(&'a'));

        list.insert_last('b');
        assert_eq!(midpoint(&list), Some(&'a'));

        list.insert_last('c');
        assert_eq!(midpoint(&list), Some(&'b'));

        list.insert_last('d');
        assert_eq!(midpoint(&list), Some(&'b'));

        list.insert_last('e');
        assert_eq!(midpoint(&list), Some(&'c'));

        list.insert_last('f');
        assert_eq!(midpoint(&list), Some(&'c'));

        list.insert_last('g');
        assert_eq!(midpoint(&list), Some(&'d'));

        list.insert_last('h');
        assert_eq!(midpoint(&list), Some(&'d'));

        list.insert_last('i');
        assert_eq!(midpoint(&list), Some(&'e'));

        list.insert_last('j');
        assert_eq!(midpoint(&list), Some(&'e'));
    }
}
