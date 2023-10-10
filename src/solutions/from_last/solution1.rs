use crate::solutions::linked_list::solution1::{LinkedList, Node};

fn from_last<T: Clone + PartialEq + std::fmt::Debug>(list: &LinkedList<T>, n: usize) -> Option<&T> {
    let mut slow: Option<&Box<Node<T>>> = list.head.as_ref();
    let mut fast = list.head.as_ref();

    for _ in 0..n {
        fast = get_next(fast);

        if fast.is_none() {
            break;
        }
    }

    if fast.is_none() {
        return None;
    }

    while get_next(fast).is_some() {
        fast = get_next(fast);
        slow = get_next(slow);
    }

    slow.and_then(|node| Some(&node.data))
}

fn get_next<T: Clone + PartialEq>(node_option: Option<&Box<Node<T>>>) -> Option<&Box<Node<T>>> {
    node_option.and_then(|node| node.next.as_ref())
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
