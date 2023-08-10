pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, data: T) {
        if data > self.data {
            if self.right.is_none() {
                let new_node = Node::new(data);
                self.right = Some(Box::new(new_node));
            } else if let Some(right) = self.right.as_mut() {
                right.insert(data);
            };
        } else if data < self.data {
            if self.left.is_none() {
                let new_node = Node::new(data);
                self.left = Some(Box::new(new_node));
            } else if let Some(left) = self.left.as_mut() {
                left.insert(data);
            };
        }
    }

    pub fn contains(&self, data: &T) -> bool {
        if self.data == *data {
            return true;
        }

        if *data > self.data {
            if let Some(right) = self.right.as_ref() {
                return right.contains(data);
            }
        } else if *data < self.data {
            if let Some(left) = self.left.as_ref() {
                return left.contains(data);
            }
        }

        return false;
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
