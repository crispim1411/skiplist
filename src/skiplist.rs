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
    head: Vec<Link<T>>, 
    level: usize,
}

impl<T: Default + Debug + PartialOrd + Clone> SkipList<T> {
    pub fn new() -> Self {
        Self { 
            head: vec![None; MAX_LEVEL], 
            level: 0 
        }
    }

    pub fn display(&self) {
        if self.head[0].is_none() {
            println!("Skiplist is empty.");
            return
        }

        for level in (0..MAX_LEVEL).rev() {
            let mut cursor = self.head[level].clone();
            while let Some(node) = cursor {
                print!("[{:?}] -> ", node.borrow().value);
                if node.borrow().forward[level].is_none() {
                    break;
                }
                cursor = node.borrow().forward[level].clone();
            }
            println!();
        }
    }

    pub fn insert(&mut self, value: T, random_level: usize) {
        if self.head[0].is_none() {
            let new_node = Node { value, forward: vec![None; random_level+1]};
            let ref_new = Rc::new(RefCell::new(new_node));
            for level in 0..=random_level {
                self.head[level] = Some(Rc::clone(&ref_new));
            }   
        }
        else {
            for level in (0..random_level).rev() {
                println!("--Level: {}--", level);
                self.recursive_insert(&self.head[level], value.clone(), level, random_level);
                println!("End level: {} - cursor: {:?}", level, &self.head[level]);
            }
        }

        if random_level > self.level {
            self.level = random_level
        }
    }

    fn recursive_insert(&self, cursor: &Link<T>, value: T, level: usize, random_level: usize) {
        println!("cursor: {:#?}", cursor);
        if let Some(node) = cursor {
            if let Some(next_node) = node.borrow().forward[level].as_ref() {
                if next_node.borrow().value < value {
                    return self.recursive_insert(&node.borrow().forward[level], value, level, random_level);
                }
            }
            let mut old_value = node.take();
            println!("node: {:#?}", old_value);
            let mut update_ref = old_value.forward;

            let mut new_node = Node { value, forward: vec![None; random_level]};
            new_node.forward[level] = update_ref[level].take();

            let ref_new = Rc::new(RefCell::new(new_node));
            update_ref[level] = Some(ref_new);
            old_value.forward = update_ref;

            *node.borrow_mut() = old_value;
        }   
    }
} 