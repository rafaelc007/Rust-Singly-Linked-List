use std::{fmt::Display, rc::{Rc, Weak}};
use std::cell::{RefCell};


/// Creates a single link object wich holds a value and a reference
/// to the next link. using reference counter to enable multiple access
/// to the next links.
/// ```
/// use singleLinkedList::Node;
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let node1 = Node::new(); // empty node
/// let node2 = Node::from(3, node1); //non-empty
/// assert_eq!(&node2.get_next().value(), &node1.value());
/// ```
#[derive(Debug)]
pub enum Node {
    Val(i32, Rc<RefCell<Node>>),
    Nil
}

impl Node {
    pub fn new() -> Self {
        Node::Nil
    }
    pub fn raw_from(val: i32, next: Node) -> Self {
        Node::Val(val, Node::ref_from(next))
    }
    pub fn ref_from(node: Node) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(node))
    }
    pub fn ref_new() -> Rc<RefCell<Node>> {
        Node::ref_from(Node::new())
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
    pub fn get_next(&self) -> &Rc<RefCell<Node>> {
        if let Node::Val(_, next) = self {
            return next;
        }
        panic!("Tried to access next for an empty node");
    }
    pub fn set_next(&mut self, node: Rc<RefCell<Node>>) {
        if let Node::Val(_, n) = self {
            n.swap(&node);
        }
    }
    pub fn take_next(&mut self) -> Node {
        if let Node::Val(_, next) = self {
            return next.take();
        }
        panic!("Tried to take next for an empty node");
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::Nil
    }
}

#[derive(Debug)]
pub struct LinkedList {
    head: Rc<RefCell<Node>>,
    tail: Weak<RefCell<Node>>,
    size: usize,
}

impl LinkedList {
    /// Creates a new empty list
    pub fn new() -> Self {
        let head = Node::ref_new();
        Self {
            head: Rc::clone(&head),
            tail: Rc::downgrade(&head),
            size: 0
        }
    }

    pub fn insert_head(&mut self, val: i32) {
        self.head.swap(&Node::ref_from(Node::raw_from(val, self.head.take())));
        if self.head.borrow().get_next().borrow().is_nil() {
            // if head points to Nil it is the last of the list
            self.tail = Rc::downgrade(&self.head);
        }
        self.size += 1;
    }
    pub fn get_head(&self) -> Rc<RefCell<Node>> {
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
        if let Some(v)= self.head.borrow().value() {
            self.head.swap(self.head.borrow().get_next());
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

pub struct ListIter {
    curr: Weak<RefCell<Node>>
}

impl ListIter {
    pub fn from(list: &LinkedList) -> Self {
        let curr = Rc::downgrade(&list.get_head());
        ListIter {curr}
    }
}

impl Iterator for ListIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let upgrade_curr = self.curr.upgrade().unwrap();
        if let Some(val) = upgrade_curr.borrow().value() {
            self.curr = Rc::downgrade(upgrade_curr.borrow().get_next());
            return Some(val)
        }
        None
    }
}