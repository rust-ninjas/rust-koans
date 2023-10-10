/// # Coding Exercise: Implementing a LinkedList
///
/// We want to implement a simple generic LinkedList with the following methods:
///
/// ## Methods
/// 1. `new() -> LinkedList<T>`: Constructor to create a new linked list.
/// 2. `insert_first(data: T)`: Adds a node to the start of the list with `data`.
/// 3. `size() -> usize`: Returns the number of nodes in the linked list.
/// 4. `get_first() -> Option<&Node<T>>`: Returns the first node in the linked list.
/// 5. `get_last() -> Option<&Node<T>>`: Returns the last node in the linked list.
/// 6. `clear()`: Empties the linked list.
/// 7. `remove_first()`: Removes the first node from the linked list.
/// 8. `remove_last()`: Removes the last node from the linked list.
/// 9. `insert_last(data: T)`: Inserts a new node at the end of the linked list.
/// 10. `get_at(index: usize) -> Option<&Node<T>>`: Get the node at a specific index.
/// 11. `remove_at(index: usize)`: Removes the node at a specific index.
/// 12. `insert_at(data: T, index: usize)`: Inserts a new node with `data` at a specific index.
/// 13. `for_each<F: FnMut(&mut Node<T>)>(mut f: F)`: Traverse the linked list, applying a function to each node.
///
/// The LinkedList uses the Node structure, which is defined as follows:
///
/// ## Node
/// ```rust
/// pub struct Node<T> {
///     data: T,
///     next: Option<Box<Node<T>>>,
/// }
/// ```
/// Note: Node is a recursive structure, where `next` points to the next Node in the LinkedList.
///
/// ## Node methods
/// 1. `new(data: T) -> Node<T>`: Constructor to create a new Node.
///
/// Constraints:
/// * All string inputs are alphanumeric and have length <= 100.
/// * The linked list will have at most 10^4 nodes.
///
/// Your task is to fill in the `LinkedList<T>` and `Node<T>` skeletons with the correct functionality for each method.
///
/// Write tests to ensure your implementation is correct.
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T: Eq + Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        todo!()
    }

    pub fn insert_first(&mut self, data: T) {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }

    pub fn get_first(&self) -> Option<&T> {
        todo!()
    }

    pub fn get_last(&self) -> Option<&T> {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn remove_first(&mut self) {
        todo!()
    }

    pub fn remove_last(&mut self) {
        todo!()
    }

    pub fn insert_last(&mut self, data: T) {
        todo!()
    }

    pub fn get_at(&self, index: usize) -> Option<&T> {
        todo!()
    }

    pub fn remove_at(&mut self, index: usize) {
        todo!()
    }

    pub fn insert_at(&mut self, data: T, index: usize) {
        todo!()
    }

    pub fn for_each<F: FnMut(&T)>(&self, mut f: F) {
        todo!()
    }
}

pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { data, next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.head.is_none());
    }

    #[test]
    fn test_get_first() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.get_first(), None); // List is empty

        list.insert_first(1);
        assert_eq!(list.get_first(), Some(&1)); // Only one element

        list.insert_first(2);
        assert_eq!(list.get_first(), Some(&2)); // Two elements, 2 is first
    }

    #[test]
    fn test_get_last() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.get_last(), None); // List is empty

        list.insert_first(1);
        assert_eq!(list.get_last(), Some(&1)); // Only one element

        list.insert_first(2);
        assert_eq!(list.get_last(), Some(&1)); // Two elements, 1 is last

        list.insert_last(3);
        assert_eq!(list.get_last(), Some(&3)); // Three elements, 3 is last
    }

    #[test]
    fn test_insert_first() {
        let mut list = LinkedList::new();
        list.insert_first('b');
        assert_eq!(list.get_first(), Some(&'b'));

        list.insert_first('a');
        assert_eq!(list.get_first(), Some(&'a'));
    }

    #[test]
    fn test_insert_last() {
        let mut list = LinkedList::new();
        list.insert_last('a');
        assert_eq!(list.get_last(), Some(&'a'));

        list.insert_last('b');
        assert_eq!(list.get_last(), Some(&'b'));
    }

    #[test]
    fn test_remove_first() {
        let mut list = LinkedList::new();
        list.insert_first(10);
        list.insert_first(20);
        list.remove_first();
        assert_eq!(list.get_first().unwrap(), &10);
        assert_eq!(list.size(), 1);
    }

    #[test]
    fn test_remove_last() {
        let mut list = LinkedList::new();
        list.insert_first(10);
        list.insert_first(20);
        list.remove_last();
        assert_eq!(list.get_last().unwrap(), &20);
        assert_eq!(list.size(), 1);
    }

    #[test]
    fn test_get_at() {
        let mut list = LinkedList::new();
        list.insert_last("first");
        list.insert_last("second");
        list.insert_last("third");

        // Test valid index
        match list.get_at(1) {
            Some(data) => assert_eq!(data, &"second"),
            None => panic!("Expected to get a node at index 1"),
        }

        // Test invalid index
        match list.get_at(5) {
            Some(_) => panic!("Expected to get None for invalid index"),
            None => assert!(true), // Passes when None is returned for invalid index
        }
    }

    #[test]
    fn test_insert_at() {
        let mut list = LinkedList::new();
        list.insert_first(10);
        list.insert_first(20);
        list.insert_at(30, 1);
        list.for_each(|val| println!("{}", val));
        assert_eq!(list.get_at(1).unwrap(), &30);
        assert_eq!(list.size(), 3);
    }

    #[test]
    fn test_remove_at() {
        let mut list = LinkedList::new();
        list.insert_first(10);
        list.insert_first(20);
        list.insert_first(30);
        list.remove_at(1);
        assert_eq!(list.get_at(1).unwrap(), &10);
        assert_eq!(list.size(), 2);
    }

    #[test]
    fn test_clear() {
        let mut list = LinkedList::new();
        list.insert_first(10);
        list.insert_first(20);
        list.clear();
        assert!(list.head.is_none());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_for_each() {
        let mut list = LinkedList::new();
        list.insert_first(1);
        list.insert_last(2);
        list.insert_last(3);

        let mut result = Vec::new();
        list.for_each(|data| result.push(data.clone() * 2));

        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_size() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.size(), 0);

        list.insert_first(1);
        assert_eq!(list.size(), 1);

        list.insert_last(2);
        assert_eq!(list.size(), 2);

        list.remove_first();
        assert_eq!(list.size(), 1);

        list.clear();
        assert_eq!(list.size(), 0);
    }
}
