use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;

const MAX_LEVEL: usize = 2;

#[derive(Default, Debug)]
struct Node<T> {
    value: T,
    forward: Vec<Link<T>>,
}
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct SkipList<T> {
    head: Link<T>, 
    level: usize,
}

impl<T: Default + Debug + PartialOrd + Clone> SkipList<T> {
    // pub fn new() -> Self {
    //     let head: Node<T> = Node { 
    //         value: Default::default(), 
    //         forward: vec![None; MAX_LEVEL] 
    //     };
    //     Self { 
    //         head: Some(Rc::new(RefCell::new(head))), 
    //         level: 0 
    //     }
    // }

    pub fn display(&self) {
        let mut cursor = self.head.clone();
        while let Some(node) = cursor {
            print!("[{:?}] -> ", node.borrow().value);
            cursor = node.borrow().forward[0].clone();
        }
        println!();
    }

    pub fn insert(&mut self, value: T) {
        for level in 0..self.level {
            println!("--Leve: {}--", level);
            self.recursive_insert(&self.head, value.clone(), level);
        }
    }

    fn recursive_insert(&self, cursor: &Link<T>, value: T, level: usize) {
        if let Some(node) = cursor {
            println!("node: {:?}", node.borrow().value);
            if let Some(next_node) = node.borrow().forward[level].as_ref() {
                println!("next_node: {:?}", next_node.borrow().value);
                if next_node.borrow().value < value {
                    return self.recursive_insert(&node.borrow().forward[level], value, level);
                }
            }
            let mut old_value = node.take();

            let update_ref = old_value.forward;

            let new_node = Node{ value, forward: update_ref};
            let ref_new = 
                Rc::new(RefCell::new(new_node));

            old_value.forward = vec![Some(Rc::clone(&ref_new)); 2];

            *node.borrow_mut() = old_value;
        }   
    }
} 

impl SkipList<u32> {
    pub fn new() -> Self {
        let mut head: Node<u32> = Node { 
            value: Default::default(), 
            forward: vec![None; MAX_LEVEL] 
        };
        let mut node1 = Node {
            value: 2,
            forward: vec![None; MAX_LEVEL]
        };
        let rc_node1 = Rc::new(RefCell::new(node1));
        head.forward = vec![Some(Rc::clone(&rc_node1)); MAX_LEVEL];
        Self { 
            head: Some(Rc::new(RefCell::new(head))), 
            level: 2 
        }
    }

}