use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;
use rand::Rng;

const MAX_LEVEL: usize = 1;

#[derive(Default, Debug)]
struct Node<T,U> {
    key: T,
    value: U,
    forward: Vec<Option<Link<T,U>>>,
}
type Link<T,U> = Rc<RefCell<Node<T,U>>>;

#[derive(Debug)]
pub struct SkipList<T,U> {
    head: Link<T,U>, 
    level: usize,
}

impl<T, U> SkipList<T,U> 
where 
    T: Default + Debug + PartialOrd,
    U: Default + Debug + Clone
{
    pub fn new() -> Self {
        let mut head = Node::default();
        head.forward = vec![None; MAX_LEVEL+1];
        Self { head: Rc::new(RefCell::new(head)), level: 0 }
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
        if self.head.borrow().forward[0].is_none() {
            println!("Skiplist is empty.");
            return
        }
        self.recursive_display(&self.head, self.level);
        println!();
    }

    fn recursive_display(&self, cursor: &Link<T,U>, level: usize) {
        if let Some(node) = cursor.borrow().forward[level].as_ref() {
            print!("[{:?}] -> ", node.borrow().key);
            return self.recursive_display(&node, level);
        }
        println!();
        if level != 0 {
            return self.recursive_display(&self.head, level-1);
        }
    }

    pub fn insert(&mut self, key: T, value: U) {
        let random_level = self.random_level();
        
        let update = self.fill_update_vector(&self.head, vec![None; random_level+1], &key, random_level);

        if let Some(node) = &update[0] {
            if let Some(next_node) = node.borrow().forward[0].as_ref() {
                if next_node.borrow().key == key {
                    println!("Item {:?} - update not implemented", key);
                    return
                }
            }
        }

        let new_node = Rc::new(RefCell::new(
            Node { 
                key, 
                value,
                forward: vec![None; random_level+1]
            }
        ));
        
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

    fn fill_update_vector(&self, cursor: &Link<T,U>, mut update: Vec<Option<Link<T,U>>>, key: &T, level: usize) -> Vec<Option<Link<T,U>>> {
        if let Some(node) = &cursor.borrow().forward[level] {
            if node.borrow().key < *key {
                return self.fill_update_vector(node, update, key, level);
            }
        }
        
        update[level] = Some(Rc::clone(cursor));

        if level > 0 {
            return self.fill_update_vector(cursor, update, key, level-1);
        }
        update
    }

    pub fn search(&self, key: T) -> Option<U> {
        if self.head.borrow().forward[0].is_none() {
            println!("Skiplist is empty.");
            return None;
        }
        return self.recursive_search(&self.head, key, self.level);
    }

    fn recursive_search(&self, cursor: &Link<T,U>, key: T, level: usize) -> Option<U> {
        if let Some(node) = cursor.borrow().forward[level].as_ref() {
            println!("cursor: {:?}", node.borrow().key);
            if node.borrow().key < key {
                return self.recursive_search(&node, key, level);
            }
            if node.borrow().key == key {
                return Some(node.borrow().value.clone())
            }
        }
        if level != 0 {
            return self.recursive_search(cursor, key, level-1);
        }
        None
    }
} 

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn is_empty_test() {
        let sl: SkipList<i16, u32> = SkipList::new();
        assert_eq!(sl.head.borrow().forward[0].is_none(), true);
    }

    #[test]
    fn reverse_insert_test() {
        let mut sl = SkipList::new();
        for i in (0..20).rev() {
            sl.insert(i, i*i);
        }
        sl.display();
    }
}