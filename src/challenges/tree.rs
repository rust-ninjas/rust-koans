/// # Node Implementation
///
/// This exercise involves creating a `Node` structure with the following requirements:
///
/// - The constructor should accept an argument that gets assigned to the data property and initialize an empty array for storing children.
/// - The `Node` should have methods `add` and `remove`.
pub struct Node<T> {
    pub data: T,
    pub children: Vec<Node<T>>,
}

impl<T: PartialEq> Node<T> {
    pub fn new(data: T) -> Self {
        todo!();
    }

    pub fn add(&mut self, data: T) {
        todo!()
    }

    pub fn remove(&mut self, data: &T) {
        todo!()
    }

    pub fn add_node(&mut self, node: Node<T>) {
        todo!()
    }
}

/// # Tree Implementation, Breadth-First Traversal, and Depth-First Traversal
///
/// This set of exercises involves working with a `Tree` structure. The following tasks should be accomplished:
///
/// - Create a `Tree` structure. The `Tree` constructor should initialize a 'root' property to `None`.
/// - Implement `traverse_bf` on the `Tree` class. This method should accept a function that gets called with each element in the tree.
/// - Implement `traverse_df` on the `Tree` class. This method should accept a function that gets called with each element in the tree.
pub struct Tree<T> {
    root: Option<Node<T>>,
}

impl<T: PartialEq> Tree<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn traverse_bf<F>(&self, mut visit: F)
    where
        F: FnMut(&T),
    {
        todo!()
    }

    pub fn traverse_df<F>(&self, mut visit: F)
    where
        F: FnMut(&T),
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_creation() {
        let node: Node<i32> = Node::new(10);
        assert_eq!(node.data, 10);
        assert!(node.children.is_empty());
    }

    #[test]
    fn node_add_child() {
        let mut node: Node<i32> = Node::new(10);
        node.add(5);
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].data, 5);
    }

    #[test]
    fn node_remove_child() {
        let mut node: Node<i32> = Node::new(10);
        node.add(5);
        node.add(6);
        node.remove(&5);
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].data, 6);
    }

    #[test]
    fn node_remove_nonexistent_child() {
        let mut node: Node<i32> = Node::new(10);
        node.add(5);
        node.remove(&6);
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].data, 5);
    }

    #[test]
    fn tree_creation() {
        let tree: Tree<i32> = Tree::new();
        assert!(tree.root.is_none());
    }

    #[test]
    fn tree_traverse_bf() {
        let mut tree: Tree<i32> = Tree::new();
        let mut node: Node<i32> = Node::new(10);
        node.add(5);
        node.add(6);
        tree.root = Some(node);
        let mut result = Vec::new();
        tree.traverse_bf(|data| result.push(*data));
        assert_eq!(result, vec![10, 5, 6]);
    }

    #[test]
    fn tree_traverse_bf_empty_tree() {
        let tree: Tree<i32> = Tree::new();
        let mut result = Vec::new();
        tree.traverse_bf(|data| result.push(*data));
        assert!(result.is_empty());
    }

    #[test]
    fn tree_traverse_df() {
        let mut tree: Tree<i32> = Tree::new();
        let mut node: Node<i32> = Node::new(10);
        node.add(5);
        node.add(6);
        tree.root = Some(node);
        let mut result = Vec::new();
        tree.traverse_df(|data| result.push(*data));
        assert_eq!(result, vec![10, 5, 6]);
    }

    #[test]
    fn tree_traverse_df_empty_tree() {
        let tree: Tree<i32> = Tree::new();
        let mut result = Vec::new();
        tree.traverse_df(|data| result.push(*data));
        assert!(result.is_empty());
    }

    #[test]
    fn tree_traverse_bf_deeper_tree() {
        let mut tree: Tree<i32> = Tree::new();
        let mut node: Node<i32> = Node::new(10);
        node.add(5);
        node.add(6);
        {
            let child_node = node.children.get_mut(0).unwrap();
            child_node.add(7);
            child_node.add(8);
        }
        tree.root = Some(node);
        let mut result = Vec::new();
        tree.traverse_bf(|data| result.push(*data));
        assert_eq!(result, vec![10, 5, 6, 7, 8]);
    }

    #[test]
    fn tree_traverse_df_deeper_tree() {
        let mut tree: Tree<i32> = Tree::new();
        let mut node: Node<i32> = Node::new(10);
        node.add(5);
        node.add(6);
        {
            let child_node = node.children.get_mut(0).unwrap();
            child_node.add(7);
            child_node.add(8);
        }
        tree.root = Some(node);
        let mut result = Vec::new();
        tree.traverse_df(|data| result.push(*data));
        assert_eq!(result, vec![10, 5, 7, 8, 6]);
    }
}
