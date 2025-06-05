#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use singleLinkedList::LinkedList;

    #[test]
    fn test_new() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_front(), None)
    }

    #[test]
    fn test_insert_head() {
        let mut list = LinkedList::new();
        list.insert_head(1);
        list.insert_head(2);
        list.insert_head(3);
        assert_eq!(vec![3,2,1], list.get_values());
    }

    #[test]
    fn test_from_vec() {
        let v = vec![3, 2, 1];
        let list: LinkedList = v.into();
        print!("{list}");
    }

    #[test]
    fn into_vec() {
        let vector = vec![1,2,3];
        let list: LinkedList = vector.clone().into();
        let list_v = list.get_values();
        assert_eq!(vector, list_v);
    }

    #[test]
    fn test_intoiter() {
        let mut vector: Vec<i32> = vec![];
        let list: LinkedList = vec![1,2,3].into();
        for n in list.into_iter() {
            if let Some(v) = n.value() {
                vector.push(v);
            }
        }
        assert_eq!(vector, vec![1,2,3]);
    }

    #[test]
    fn test_insert_tail() {
        let mut list = LinkedList::new();
        list.insert_tail(1);
        list.insert_tail(2);
        list.insert_tail(3);
        assert_eq!(vec![1,2,3], list.get_values());
    }

    #[test]
    fn test_remove() {
        let mut list: LinkedList = vec![3,2,1].into();
        list.remove(1);
        println!("{}", list);
    }
}