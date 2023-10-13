use crate::solutions::tree::solution1::Node;

pub fn level_width<T>(root: &Node<T>) -> Vec<usize> {
    let mut levels: Vec<usize> = vec![1];

    let mut children: Vec<&Node<T>> = root.children.iter().collect();

    while !children.is_empty() {
        levels.push(children.len());

        children = children
            .iter()
            .flat_map(|child| child.children.iter())
            .collect()
    }

    levels
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
