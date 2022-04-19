use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;
use rand::Rng;

const MAX_LEVEL: usize = 1;

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
    pub fn new() -> Self {
        let mut head = Node::default();
        head.forward = vec![None; MAX_LEVEL+1];
        Self { head: Some(Rc::new(RefCell::new(head))), level: 0 }
    }

    fn random_level(&self) -> usize{
        let mut level = 0;
        let mut rng = rand::thread_rng();
        while rng.gen::<f32>() < 0.5 && level < MAX_LEVEL {
            level += 1;
        }
        level
    }

    pub fn display(&self) {
        if self.head.as_ref().unwrap().borrow().forward[0].is_none() {
            println!("Skiplist is empty.");
            return
        }
        self.recursive_display(&self.head.as_ref().unwrap(), self.level);
        println!();
    }

    fn recursive_display(&self, cursor: &Rc<RefCell<Node<T>>>, level: usize) {
        if let Some(node) = cursor.borrow().forward[level].as_ref() {
            print!("[{:?}({})] -> ", node.borrow().value, Rc::strong_count(&node));
            return self.recursive_display(&node, level);
        }
        println!();
        if level != 0 {
            return self.recursive_display(&self.head.as_ref().unwrap(), level-1);
        }
    }

    pub fn insert(&mut self, value: T) {
        let random_level = self.random_level();
        
        let update = self.fill_update_vector(&self.head.as_ref().unwrap(), vec![None; random_level+1], &value, random_level);

        if let Some(node) = &update[0] {
            if let Some(next_node) = node.borrow().forward[0].as_ref() {
                if next_node.borrow().value == value {
                    println!("Item {:?} já cadastrado", value);
                    return
                }
            }
        }

        let new_node = Rc::new(RefCell::new(Node { 
            value: value, 
            forward: vec![None; random_level+1]
        }));
        
        for level in 0..=random_level {
            let node = update[level].as_ref().unwrap();

            let mut new_node_inner = new_node.take();
            let mut node_inner = node.take();
            
            new_node_inner.forward[level] = node_inner.forward[level].take();
            new_node.replace(new_node_inner);

            node_inner.forward[level] = Some(Rc::clone(&new_node));
            node.replace(node_inner);
        }
        
        if random_level > self.level {
            self.level = random_level
        }
    }

    fn fill_update_vector(&self, cursor: &Rc<RefCell<Node<T>>>, mut update: Vec<Link<T>>, value: &T, level: usize) -> Vec<Link<T>> {
        if let Some(node) = &cursor.borrow().forward[level] {
            if node.borrow().value < *value {
                return self.fill_update_vector(node, update, value, level);
            }
        }
        
        update[level] = Some(Rc::clone(cursor));

        if level > 0 {
            return self.fill_update_vector(cursor, update, value, level-1);
        }
        update
    }
} 