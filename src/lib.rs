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
/// let node1 = Node::raw_from(2, Node::new());
/// let node2 = Node::raw_from(3, node1); //non-empty
/// let next_node2_val = &node2.get_next().borrow().value();
/// assert_eq!(next_node2_val.unwrap(), 2);
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

    pub fn tail(&self) -> i32 {
        self.tail.upgrade().unwrap().borrow().value().unwrap()
    }

    pub fn insert_tail(&mut self, val: i32) {
        let rc_tail = self.tail.upgrade().unwrap();
        if rc_tail.borrow().is_nil() {
            self.insert_head(val);
        } else {
            let new_tail = Node::ref_from(
                Node::raw_from(val, rc_tail.borrow().get_next().take()));
            rc_tail.borrow_mut().set_next(new_tail);
            self.tail = Rc::downgrade(rc_tail.borrow().get_next());
            self.size += 1;
        }
    }

    pub fn remove(&mut self, idx: usize) -> bool {
        if idx > self.size {
            return false
        }
        if idx == 0 {return self.pop_front().is_some()}
        let prev_node = 
            match self.node_at(idx-1) {
                Ok(v) => v,
                Err(_) => {return false;}
            };
        
        //replace node references here
        if !prev_node.borrow().is_nil() {
            let mut idx_ref = prev_node.borrow_mut().take_next();
            if !idx_ref.is_nil() {
                prev_node.borrow_mut().set_next(Node::ref_from(idx_ref.take_next()));
            }
            true
        } else {false}
    }

    pub fn into_iter(self) -> ListIntoIter {
        ListIntoIter(self)
    }
    pub fn iter(&self) -> ListIter {
        ListIter::from(self)
    }
    pub fn iter_vals(&self) -> ListValIter {
        ListValIter(self.iter())
    }
    pub fn get_values(&self) -> Vec<i32> {
        self.into()
    }
    fn node_at(&self, idx: usize) -> Result<Rc<RefCell<Node>>, &'static str> {
        let mut i = 0_usize;
        for n in self.iter() {
            if i == idx {
                return Ok(n.upgrade().unwrap())
            }
            i += 1;
        }
        Err("Out of bounds")
    }
    pub fn pop_front(&mut self) -> Option<i32> {
        if let Some(v)= self.head.borrow().value() {
            self.head.swap(self.head.borrow().get_next());
            return Some(v);
        }
        None
    }
    // Removes node from head and returns it
    pub fn take_head(&mut self) -> Node {
        let old_head = self.head.take();
        if !old_head.is_nil() {
            self.head.swap(old_head.get_next());
        }
        old_head
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::new();
        for v in self.iter_vals() {
            out_str.push_str(&format!("{v} "));
        }
        out_str.pop();
        write!(f, "({})", out_str)
    }
}

pub struct ListIntoIter(LinkedList);

impl Iterator for ListIntoIter {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        let ret_node = self.0.take_head();
        if ret_node.is_nil() {
            None
        } else {Some(ret_node)}
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

impl From<LinkedList> for Vec<i32> {
    fn from(list: LinkedList) -> Self {
        Self::from(&list)
    }
}

impl From<&LinkedList> for Vec<i32> {
    fn from(list: &LinkedList) -> Self {
        let mut vector = vec!();
        for val in list.iter_vals() {
            vector.push(val);
        }
        vector
    }
}

/// creates a list iterator to access nodes in for loops
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
    type Item = Weak<RefCell<Node>>;
    fn next(&mut self) -> Option<Self::Item> {
        let upgrade_curr = self.curr.upgrade().unwrap();
        if !upgrade_curr.borrow().is_nil() {
            self.curr = Rc::downgrade(&upgrade_curr.borrow().get_next());
            return Some(Rc::downgrade(&upgrade_curr))
        }
        None
    }
}

/// creates a iterator to get values on for loops
pub struct ListValIter(ListIter);

impl Iterator for ListValIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_node) = self.0.next() {
            return next_node.upgrade().unwrap().borrow().value()
        }
        None
    }
}