use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;
use rand::Rng;

#[derive(Default)]
struct Node<T,U> {
    key: T,
    value: U,
    forward: Vec<Option<Link<T,U>>>,
}
type Link<T,U> = Rc<RefCell<Node<T,U>>>;

pub struct SkipList<T,U> {
    head: Link<T,U>, 
    level: usize,
    max_level: usize,
}

impl<T, U> SkipList<T,U> 
where 
    T: Default + Debug + PartialOrd,
    U: Default + Debug + Clone
{
    pub fn new(max_level: usize) -> Self {
        let mut head = Node::default();
        head.forward = vec![None; max_level+1];
        Self { 
            head: Rc::new(RefCell::new(head)), 
            level: 0,
            max_level 
        }
    }

    fn random_level(&self) -> usize{
        let mut level = 0;
        let mut rng = rand::thread_rng();
        while rng.gen::<f32>() < 0.5 && level < self.max_level {
            level += 1;
        }
        level
    }

    pub fn display(&self) {
        if self.head.borrow().forward[0].is_none() {
            println!("Skiplist is empty.");
            return
        }
        self.display_recursive(&self.head, self.level);
        println!();
    }

    fn display_recursive(&self, cursor: &Link<T,U>, level: usize) {
        if let Some(node) = cursor.borrow().forward[level].as_ref() {
            print!("[{:?}] -> ", node.borrow().key);
            return self.display_recursive(&node, level);
        }
        println!();
        if level != 0 {
            return self.display_recursive(&self.head, level-1);
        }
    }

    pub fn insert(&mut self, key: T, value: U) {
        let random_level = self.random_level();
        
        let update = self.fill_update_vector(&self.head, vec![None; random_level+1], &key, random_level);

        if let Some(node) = &update[0] {
            if let Some(next_node) = node.borrow().forward[0].as_ref() {
                if next_node.borrow().key == key {
                    println!("Item {:?} already inserted", key);
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

    fn fill_update_vector(&self, 
        cursor: &Link<T,U>, 
        mut update: Vec<Option<Link<T,U>>>, 
        key: &T, 
        level: usize) -> Vec<Option<Link<T,U>>> {

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
        return self.search_recursive(&self.head, key, self.level);
    }

    fn search_recursive(&self, cursor: &Link<T,U>, key: T, level: usize) -> Option<U> {
        if let Some(node) = cursor.borrow().forward[level].as_ref() {
            if node.borrow().key < key {
                return self.search_recursive(&node, key, level);
            }
            if node.borrow().key == key {
                return Some(node.borrow().value.clone())
            }
        }
        if level != 0 {
            return self.search_recursive(cursor, key, level-1);
        }

        None
    }

    pub fn delete(&mut self, key: T) {
        let update = self.fill_update_vector(&self.head, vec![None; self.level+1], &key, self.level);

        let mut option_delete = None;
        if let Some(update_node) = update[0].as_ref() {
            if let Some(next_node) = update_node.borrow().forward[0].as_ref() {
                if next_node.borrow().key == key {
                    option_delete = Some(Rc::clone(next_node));
                }
            }
        }
        
        if let Some(delete_node) = option_delete {
            for level in 0..=self.level {                

                let cursor = update[level].as_ref().unwrap();
                if cursor.borrow().forward[level].is_none() {
                    break;
                }
                else if !Rc::ptr_eq(
                    cursor.borrow().forward[level].as_ref().unwrap(), 
                    &delete_node) {
                    break;
                }

                let mut delete_node_inner = delete_node.take();
                let mut node_inner = cursor.take();

                node_inner.forward[level] = delete_node_inner.forward[level].take();

                cursor.replace(node_inner);
                delete_node.replace(delete_node_inner);
            }
            while self.level > 1 && self.head.borrow().forward[self.level].is_none() {
                self.level -= 1;
            }
        }
        else {
            println!("Item {:?} not found", key);
        }
    }
} 

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    fn is_empty_test() {
        let sl: SkipList<i16, u32> = SkipList::new(2);
        assert_eq!(sl.head.borrow().forward[0].is_none(), true);
    }

    #[test]
    fn reverse_insert_test() {
        let mut sl = SkipList::new(2);
        for i in (0..20).rev() {
            sl.insert(i, i*i);
        }
        for i in 0..20 {
            assert_eq!(i*i, sl.search(i).unwrap());
        }
    }

    #[test]
    fn delete_test() {
        let mut sl = SkipList::new(2);
        for i in (0..20).rev() {
            sl.insert(i, i*i);
        }
        for i in (0..20).step_by(4) {
            sl.delete(i);
        }
        for i in (0..20).step_by(4) {
            assert!(sl.search(i).is_none());
        }
    }
}