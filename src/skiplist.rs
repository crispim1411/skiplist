use std::borrow::BorrowMut;
use std::ops::Add;
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

impl<T: Default + Debug + PartialOrd + Clone + Add<Output = T>> SkipList<T> {
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
        self.recursive_display(self.head[MAX_LEVEL-1].as_ref(), MAX_LEVEL-1);
        println!("\n");
    }

    fn recursive_display(&self, cursor: Option<&Rc<RefCell<Node<T>>>>, mut level: usize) {
        if let Some(node) = cursor {
            print!("[{:?}({})] -> ", node.borrow().value, Rc::strong_count(&node));
            return self.recursive_display(node.borrow().forward[level].as_ref(), level);
        }
        if level != 0 {
            level -= 1;
            println!();
            self.recursive_display(self.head[level].as_ref(), level);
        }
    }

    pub fn insert(&mut self, value: T, random_level: usize) {
        let new_node = Rc::new(RefCell::new(Node { 
            value: value.clone(), 
            forward: vec![None; random_level+1]
        }));

        if self.head[0].is_none() {
            for level in 0..=random_level {
                self.head[level] = Some(Rc::clone(&new_node));
            }   
        }
        else if self.head[0].as_ref().unwrap().borrow().value > value {
            let mut inner_value = new_node.take();
            inner_value.forward = self.head[0..=random_level].to_vec();
            new_node.replace(inner_value);

            for level in 0..=random_level {
                self.head[level] = Some(Rc::clone(&new_node));
            }
        }
        else {
            let cursor = &self.head[random_level].as_ref().unwrap();
            self.recursive_insert(cursor, &new_node, random_level);
        }

        if random_level > self.level {
            self.level = random_level
        }
    }

    fn recursive_insert(&self, cursor: &Rc<RefCell<Node<T>>>, new_node: &Rc<RefCell<Node<T>>>, level: usize) {
        if let Some(next_node) = cursor.borrow().forward[level].as_ref() {
            if next_node.borrow().value < new_node.borrow().value {
                return self.recursive_insert(
                    &cursor.borrow().forward[level].as_ref().unwrap(), new_node, level);
            }
        }
        let mut old_value = cursor.take();
        let mut inner_value = new_node.take();

        inner_value.forward.splice(0..=level, old_value.forward[0..=level].to_vec());
        new_node.replace(inner_value);
        old_value.forward[level] = Some(Rc::clone(new_node));
        cursor.replace(old_value);
        
        if level != 0 {
            self.recursive_insert(cursor, &new_node, level-1);
        }
    }
} 