use crate::challenges::tree::Node;

/// # Level Width
///
/// This is an exercise in calculating the width of a tree at each level.
///
/// The task is to create a function `level_width` that takes the root node of a tree as input and
/// returns a vector where each element represents the width (number of nodes) at each level of the tree.
///
/// ## Required
/// Solve tree exercise first and use the Node from it here.
///
/// ## Example
///
/// Consider the following tree:
///
/// ```text
///     0
///   / |  \
///  1  2   3
///  |      |
///  4      5
/// ```
///
/// The function should return: `[1, 3, 2]`.
///
pub fn level_width<T>(root: &Node<T>) -> Vec<usize> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_width_single_node() {
        let root: Node<i32> = Node::new(0);
        assert_eq!(level_width(&root), vec![1]);
    }

    #[test]
    fn level_width_two_levels() {
        let mut root: Node<i32> = Node::new(0);
        root.add(1);
        root.add(2);
        root.add(3);
        assert_eq!(level_width(&root), vec![1, 3]);
    }

    #[test]
    fn level_width_three_levels() {
        let mut root: Node<i32> = Node::new(0);
        root.add(1);
        root.add(2);
        root.add(3);
        {
            let child_node = root.children.get_mut(0).unwrap();
            child_node.add(4);
        }
        {
            let child_node = root.children.get_mut(2).unwrap();
            child_node.add(5);
        }
        assert_eq!(level_width(&root), vec![1, 3, 2]);
    }

    #[test]
    fn level_width_four_levels() {
        let mut root: Node<i32> = Node::new(0);
        root.add(1);
        root.add(2);
        root.add(3);
        {
            let child_node = root.children.get_mut(0).unwrap();
            child_node.add(4);
        }
        {
            let child_node = root.children.get_mut(2).unwrap();
            child_node.add(5);
            let grand_child_node = child_node.children.get_mut(0).unwrap();
            grand_child_node.add(6);
        }
        assert_eq!(level_width(&root), vec![1, 3, 2, 1]);
    }
}
