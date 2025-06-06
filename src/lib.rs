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
pub enum Node<T> {
    Val(T, Rc<RefCell<Node<T>>>),
    Nil
}

impl<T> Node<T>
where
    T: Copy+Clone {
    pub fn new() -> Self {
        Node::Nil
    }
    pub fn raw_from(val: T, next: Node<T>) -> Self {
        Node::Val(val, Node::ref_from(next))
    }
    pub fn ref_from(node: Node<T>) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(node))
    }
    pub fn ref_new() -> Rc<RefCell<Node<T>>> {
        Node::ref_from(Node::new())
    }
    pub fn is_nil(&self) -> bool {
        match self {
            Node::Nil => true,
            Node::Val(_, _) => false
        }
    }
    pub fn value(&self) -> Option<T> {
        if let Node::Val(n, _) = self {
            return Some(*n)
        }
        None
    }
    pub fn set_val(&mut self, val: T) {
        if let Node::Val(v, _) = self {
            *v = val;
        }
    }
    /// Provides a reference to the next node
    pub fn get_next(&self) -> &Rc<RefCell<Node<T>>> {
        if let Node::Val(_, next) = self {
            return next;
        }
        panic!("Tried to access next for an empty node");
    }
    pub fn set_next(&mut self, node: Rc<RefCell<Node<T>>>) {
        if let Node::Val(_, n) = self {
            n.swap(&node);
        }
    }
    pub fn take_next(&mut self) -> Node<T> {
        if let Node::Val(_, next) = self {
            return next.take();
        }
        panic!("Tried to take next for an empty node");
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node::Nil
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Rc<RefCell<Node<T>>>,
    tail: Weak<RefCell<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T>
where
    T: Copy+Clone {
    /// Creates a new empty list
    pub fn new() -> Self {
        let head = Node::ref_new();
        Self {
            head: Rc::clone(&head),
            tail: Rc::downgrade(&head),
            size: 0
        }
    }

    pub fn insert_head(&mut self, val: T) {
        self.head.replace(Node::raw_from(val, self.head.take()));
        if self.head.borrow().get_next().borrow().is_nil() {
            // if head points to Nil it is the last of the list
            self.tail = Rc::downgrade(&self.head);
        }
        self.size += 1;
    }

    pub fn get_head(&self) -> Rc<RefCell<Node<T>>> {
        Rc::clone(&self.head)
    }

    pub fn tail(&self) -> T {
        self.tail.upgrade().unwrap().borrow().value().unwrap()
    }

    pub fn insert_tail(&mut self, val: T) {
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

    pub fn into_iter(self) -> ListIntoIter<T> {
        ListIntoIter(self)
    }
    pub fn iter(&self) -> ListIter<T> {
        ListIter::from(self)
    }
    pub fn iter_vals(&self) -> ListValIter<T> {
        ListValIter(self.iter())
    }
    pub fn get_values(&self) -> Vec<T> {
        self.into()
    }
    fn node_at(&self, idx: usize) -> Result<Rc<RefCell<Node<T>>>, &'static str> {
        let mut i = 0_usize;
        for n in self.iter() {
            if i == idx {
                return Ok(n.upgrade().unwrap())
            }
            i += 1;
        }
        Err("Out of bounds")
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(v)= self.head.borrow().value() {
            self.head.swap(self.head.borrow().get_next());
            return Some(v);
        }
        None
    }
    // Removes node from head and returns it
    pub fn take_head(&mut self) -> Node<T> {
        let old_head = self.head.take();
        if !old_head.is_nil() {
            self.head.swap(old_head.get_next());
        }
        old_head
    }
}

impl<T> Display for LinkedList<T>
where
    T: Clone+Copy+Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::new();
        for v in self.iter_vals() {
            out_str.push_str(&format!("{v} "));
        }
        out_str.pop();
        write!(f, "({})", out_str)
    }
}

pub struct ListIntoIter<T>(LinkedList<T>);

impl<T> Iterator for ListIntoIter<T>
where
    T:Clone+Copy {
    type Item = Node<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let ret_node = self.0.take_head();
        if ret_node.is_nil() {
            None
        } else {Some(ret_node)}
    }
}

impl<T> From<Vec<T>> for LinkedList<T>
where
    T:Clone+Copy {
    fn from(values: Vec<T>) -> Self {
        let mut list = Self::new();
        for val in values.iter().rev() {
            list.insert_head(*val);
        }
        list
    }
}

impl<T> From<LinkedList<T>> for Vec<T>
where
    T:Clone+Copy {
    fn from(list: LinkedList<T>) -> Self {
        Self::from(&list)
    }
}

impl<T> From<&LinkedList<T>> for Vec<T>
where
    T:Clone+Copy {
    fn from(list: &LinkedList<T>) -> Self {
        let mut vector = vec!();
        for val in list.iter_vals() {
            vector.push(val);
        }
        vector
    }
}

/// creates a list iterator to access nodes in for loops
pub struct ListIter<T> {
    curr: Weak<RefCell<Node<T>>>
}

impl<T> ListIter<T>
where
    T:Clone+Copy {
    pub fn from(list: &LinkedList<T>) -> Self {
        let curr = Rc::downgrade(&list.get_head());
        ListIter {curr}
    }
}

impl<T> Iterator for ListIter<T>
where
    T:Clone+Copy {
    type Item = Weak<RefCell<Node<T>>>;
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
pub struct ListValIter<T>(ListIter<T>);

impl<T> Iterator for ListValIter<T>
where
    T:Clone+Copy {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_node) = self.0.next() {
            return next_node.upgrade().unwrap().borrow().value()
        }
        None
    }
}