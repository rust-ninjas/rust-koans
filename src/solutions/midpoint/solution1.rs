use crate::solutions::linked_list::solution1::LinkedList;

fn midpoint<T: Clone + PartialEq + Eq>(list: &LinkedList<T>) -> Option<&T> {
    let mut slow = list.head.as_ref();
    let mut fast = list
        .head
        .as_ref()
        .and_then(|node| node.next.as_ref())
        .and_then(|node| node.next.as_ref());

    while slow.is_some() && fast.is_some() {
        slow = slow.and_then(|node| node.next.as_ref());
        fast = fast
            .and_then(|node| node.next.as_ref())
            .and_then(|node| node.next.as_ref());
    }

    slow.and_then(|node| Some(&node.data))
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
