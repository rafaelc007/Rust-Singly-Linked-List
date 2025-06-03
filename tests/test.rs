#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use singleLinkedList::LinkedList;

    #[test]
    fn test_new() {
        let list = LinkedList::new();
        // assert_eq!(list.pop_front().value(), None)
    }

    #[test]
    fn test_insert_head() {
        let mut list = LinkedList::new();
        list.insert_head(1);
        list.insert_head(2);
        list.insert_head(3);
        println!("{:?}", &list);
    }

    #[test]
    fn test_from_vec() {
        let v = vec![3, 2, 1];
        let list: LinkedList = v.into();
        print!("{list}");
    }

    #[test]
    fn test_intoiter() {
        let list: LinkedList = vec![3, 2, 1].into();
        print!("{list}");
    }

    #[test]
    fn test_insert_tail() {
        let mut list = LinkedList::new();
        list.insert_tail(1);
        list.insert_tail(2);
        list.insert_tail(3);
        print!("{list}");
    }
}