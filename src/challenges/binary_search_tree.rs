/// # Exercise: Binary Search Tree Implementation in Rust
///
/// 1. Implement the `Node<T>` struct to represent a node in a binary search tree. Each `Node` has three properties:
///     - `data`: The value of the node.
///     - `left`: Left child node.
///     - `right`: Right child node.
///
/// 2. Implement the `insert` method for the `Node<T>` struct. This method should:
///     - Take 'data' as an argument.
///     - Insert a new `Node` with the given 'data' at the appropriate position in the binary search tree.
///
/// 3. Implement the `contains` method for the `Node<T>` struct. This method should:
///     - Take 'data' as an argument.
///     - Return a boolean indicating whether a `Node` with this 'data' exists in the tree.
pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new(data: T) -> Self {
        todo!();
    }

    pub fn insert(&mut self, data: T) {
        todo!();
    }

    pub fn contains(&self, data: &T) -> bool {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new(10);
        assert_eq!(node.data, 10);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_insert() {
        let mut root = Node::new(10);

        root.insert(5);
        assert_eq!(root.left.as_ref().unwrap().data, 5);

        root.insert(15);
        assert_eq!(root.right.as_ref().unwrap().data, 15);

        root.insert(3);
        assert_eq!(root.left.as_ref().unwrap().left.as_ref().unwrap().data, 3);

        root.insert(7);
        assert_eq!(root.left.as_ref().unwrap().right.as_ref().unwrap().data, 7);

        root.insert(13);
        assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().data, 13);

        root.insert(17);
        assert_eq!(
            root.right.as_ref().unwrap().right.as_ref().unwrap().data,
            17
        );
    }

    #[test]
    fn test_contains() {
        let mut root = Node::new(10);
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(13);
        root.insert(17);

        assert!(root.contains(&10));
        assert!(root.contains(&5));
        assert!(root.contains(&15));
        assert!(root.contains(&3));
        assert!(root.contains(&7));
        assert!(root.contains(&13));
        assert!(root.contains(&17));

        assert!(!root.contains(&0));
        assert!(!root.contains(&20));
        assert!(!root.contains(&6));
    }
}
