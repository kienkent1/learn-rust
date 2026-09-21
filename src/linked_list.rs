use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Node<T> {
    pub val: Option<T>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    fn convert_node(node: Node<T>) -> Option<Rc<RefCell<Node<T>>>> {
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn add_begin(&mut self, value: T) {
        let new_head = Self::convert_node(Node {
            val: Some(value),
            next: self.head.clone(),
            prev: None,
        });

        match &self.head {
            Some(old_head) => {
                old_head.borrow_mut().prev = new_head.clone();
            }
            None => {
                self.tail = new_head.clone();
            }
        }
        self.head = new_head;
    }
    pub fn add_end(&mut self, value: T) {
        let new_tail = Self::convert_node(Node {
            val: Some(value),
            next: None,
            prev: self.tail.clone(),
        });

        match &self.tail {
            Some(old_tail) => {
                old_tail.borrow_mut().next = new_tail.clone();
            }
            None => {
                self.tail = new_tail.clone();
            }
        }
        self.tail = new_tail;
    }
    pub fn print(&self) {
        let mut current = self.head.clone();

        while let Some(node_rc) = current {
            let node = node_rc.borrow();
            if let Some(ref value) = node.val {
                print!("{:?} <-> ", value);
            }

            current = node.next.clone();
        }

        println!("None");
    }
}

fn main() {
    let mut list = LinkedList {
        head: None,
        tail: None,
    };
    list.add_begin(3);
    list.add_end(4);
    list.add_end(5);
    list.add_end(6);

    list.print();
}
