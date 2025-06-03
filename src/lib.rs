use std::{fmt::Display, rc::{Rc, Weak}};


/// Creates a single link object wich holds a value and a reference
/// to the next link. using reference counter to enable multiple access
/// to the next links.
/// ```
/// use singleLinkedList::Node;
/// use std::rc::Rc;
///
/// let node1 = Rc::new(Node::new()); // empty node
/// let node2 = Node::from(3, &node1); //non-empty
/// assert_eq!(&node2.get_next().value(), &node1.value());
/// ```
#[derive(Debug)]
pub enum Node {
    Val(i32, Rc<Node>),
    Nil
}

impl Node {
    pub fn new() -> Self {
        Node::Nil
    }
    pub fn from(val: i32, next: &Rc<Node>) -> Self {
        Node::Val(val, Rc::clone(next))
    }
    pub fn is_nil(&self) -> bool {
        match self {
            Node::Nil => true,
            Node::Val(_, _) => false
        }
    }
    pub fn value(&self) -> Option<i32> {
        if let Node::Val(n, _) = self {
            return Some(*n)
        }
        None
    }
    pub fn set_val(&mut self, val: i32) {
        if let Node::Val(v, _) = self {
            *v = val;
        }
    }
    /// Provides a reference to the next node
    pub fn get_next(&self) -> Rc<Node> {
        if let Node::Val(_, next) = self {
            return Rc::clone(next);
        }
        Node::Nil.into()
    }
    pub fn set_next(&mut self, node: Rc<Node>) {
        if let Node::Val(_, n) = self {
            *n = Rc::clone(&node);
        }
    }
}

#[derive(Debug)]
pub struct LinkedList {
    head: Rc<Node>,
    tail: Weak<Node>,
    size: usize,
}

impl LinkedList {
    /// Creates a new empty list
    pub fn new() -> Self {
        let head = Rc::new(Node::new());
        Self {
            head: Rc::clone(&head),
            tail: Rc::downgrade(&head),
            size: 0
        }
    }

    pub fn insert_head(&mut self, val: i32) {
        self.head = Rc::new(Node::from(val, &self.head));
        if let Node::Nil = *self.head.get_next() {
            // if head points to Nil it is the last of the list
            self.tail = Rc::downgrade(&self.head);
        }
        self.size += 1;
    }
    pub fn get_head(&self) -> Rc<Node> {
        Rc::clone(&self.head)
    }
    pub fn insert_tail(&mut self, val: i32) {
        
    }
    pub fn into_iter(self) -> ListIntoIter {
        ListIntoIter(self)
    }
    pub fn iter(&self) -> ListIter {
        ListIter::from(self)
    }
    // pub fn insert_at(&mut self, index: usize, val: i32) -> bool {
    //     false
    // }
    // pub fn remove(&mut self, idx: usize) -> bool {false}

    // pub fn get_values(&self) -> Vec<i32> {
    //     self.into()
    // }
    pub fn pop_front(&mut self) -> Option<i32> {
        if let Some(v)= self.head.value() {
            self.head = self.head.get_next();
            return Some(v);
        }
        None
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::new();
        for v in self.iter() {
            out_str.push_str(&format!("{v} "));
        }
        out_str.pop();
        write!(f, "({})", out_str)
    }
}

pub struct ListIntoIter(LinkedList);

impl Iterator for ListIntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl From<Vec<i32>> for LinkedList {
    fn from(values: Vec<i32>) -> Self {
        let mut list = Self::new();
        for val in values.iter().rev() {
            list.insert_head(*val);
        }
        list
    }
}

// impl From<LinkedList> for Vec<i32> {
//     fn from(list: LinkedList) -> Self {
//         let mut vector = vec!();
//         for val in list.into_iter() {
//             vector.push(val);
//         }
//         vector
//     }
// }

// impl From<&LinkedList> for Vec<i32> {
//     fn from(list: &LinkedList) -> Self {
//         let mut vector = vec!();
//         for val in list.into_iter() {
//             vector.push(val);
//         }
//         vector
//     }
// }

pub struct ListIter<'a> {
    list: &'a LinkedList,
    curr: Weak<Node>
}

impl<'a> ListIter<'a> {
    pub fn from(list: &'a LinkedList) -> Self {
        let curr = Rc::downgrade(&list.get_head());
        ListIter { list, curr}
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.curr.upgrade() {
            self.curr = Rc::downgrade(&node.get_next());
            return node.value()
        }
        None
    }
}