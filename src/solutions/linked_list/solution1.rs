pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T: Eq + Clone> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn insert_first(&mut self, data: T) {
        let old_head = self.head.take();
        self.head = Some(Box::new(Node::new(data, old_head)));
    }

    pub fn size(&self) -> usize {
        let mut size: usize = 0;
        let mut optional_next = self.head.as_ref();

        while let Some(next) = optional_next {
            optional_next = next.next.as_ref();

            size += 1;
        }

        size
    }

    pub fn get_first(&self) -> Option<&T> {
        self.head.as_ref().and_then(|node| Some(&node.data))
    }

    pub fn get_last(&self) -> Option<&T> {
        let mut current_node = self.head.as_ref();

        while let Some(node) = current_node {
            if node.next.is_none() {
                return Some(&node.data);
            }

            current_node = node.next.as_ref();
        }

        None
    }

    pub fn clear(&mut self) {
        self.head.take();
    }

    pub fn remove_first(&mut self) {
        self.head = self.head.take().and_then(|head| head.next);
    }

    pub fn remove_last(&mut self) {
        let mut current_node = self.head.as_mut();

        if current_node.is_none() {
            current_node.take();
            return;
        }

        while let Some(node) = current_node.as_mut() {
            if node
                .next
                .as_ref()
                .and_then(|next| next.next.as_ref())
                .is_none()
            {
                node.next.take();
                break;
            }
        }
    }

    pub fn insert_last(&mut self, data: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(data, None)));

            return;
        }

        let mut current_node = &mut self.head;

        while let Some(node) = current_node {
            if node.next.is_none() {
                let new_node = Box::new(Node::new(data, None));

                node.next = Some(new_node);

                break;
            }
            current_node = &mut node.next;
        }
    }

    pub fn get_at(&self, index: usize) -> Option<&T> {
        let mut current_node = &self.head;
        let mut current_index: usize = 0;

        while let Some(node) = current_node {
            if current_index == index {
                return Some(&node.data);
            }

            current_index += 1;
            current_node = &node.next;
        }

        None
    }

    pub fn remove_at(&mut self, index: usize) {
        if index == 0 {
            self.head = self.head.take().and_then(|head| head.next);
            return;
        }

        let mut current_node = &mut self.head;
        let mut current_index: usize = 0;

        while let Some(node) = current_node {
            if current_index == index - 1 {
                node.next = node.next.take().and_then(|next| next.next);

                break;
            }

            current_index += 1;
            current_node = &mut node.next;
        }
    }

    pub fn insert_at(&mut self, data: T, index: usize) {
        if index == 0 {
            let next = self.head.take();

            self.head = Some(Box::new(Node::new(data, next)));
            return;
        }

        let mut current_node = &mut self.head;
        let mut current_index: usize = 0;

        while let Some(node) = current_node {
            if current_index == index - 1 {
                let new_node = Box::new(Node::new(data, node.next.take()));

                node.next = Some(new_node);
                break;
            }

            current_index += 1;
            current_node = &mut node.next;
        }
    }

    pub fn for_each<F: FnMut(&T)>(&self, mut f: F) {
        let mut current_node = &self.head;

        while let Some(node) = current_node {
            f(&node.data);
            current_node = &node.next;
        }
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
