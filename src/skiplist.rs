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
    head: Vec<Link<T>>, 
    level: usize,
}

impl<T: Default + Debug + PartialOrd + Clone> SkipList<T> {
    pub fn new() -> Self {
        Self { 
            head: vec![None; MAX_LEVEL+1], 
            level: 0 
        }
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
        if self.head[0].is_none() {
            println!("Skiplist is empty.");
            return
        }
        self.recursive_display(self.head[self.level].as_ref(), self.level);
        println!("\n");
    }

    fn recursive_display(&self, cursor: Option<&Rc<RefCell<Node<T>>>>, mut level: usize) {
        if let Some(node) = cursor {
            print!("[{:?}({}) lvl{}] -> ", node.borrow().value, Rc::strong_count(&node), level);
            return self.recursive_display(node.borrow().forward[level].as_ref(), level);
        }
        if level != 0 {
            level -= 1;
            println!();
            self.recursive_display(self.head[level].as_ref(), level);
        }
    }

    pub fn insert2(&mut self, value: T) {
        let mut level = self.level;
        loop {
            if let Some(node) = &self.head[level] {
                if value >= node.borrow().value {
                    break;
                }
            }
            if level == 0 { break; }
            level -= 1;
        }

        let cursor = self.head[level].as_ref().unwrap();
        let mut update = self.get_mut_recursive(cursor, self.head.clone(), &value, level);

        if let Some(node) = &update[0] {
            if let Some(forward) = node.borrow().forward[0].as_ref() {
                if forward.borrow().value == value {
                    println!("Item já cadastrado");
                    return
                }
            }
        }
        let random_level = self.random_level();
        let new_node = Rc::new(RefCell::new(Node { 
            value: value, 
            forward: vec![None; random_level+1]
        }));
        for level in 0..=random_level {
            if new_node.borrow().value < update[level].as_ref().unwrap().borrow().value {
                let mut inner_value = new_node.take();
                inner_value.forward[level] = update[level].take();
                new_node.replace(inner_value);
                self.head[level] = Some(Rc::clone(&new_node));
                
            } else {
                println!("{:?} <- {:?}", new_node.borrow().forward[level], update[level].as_ref().unwrap().borrow().forward[level].as_ref().unwrap().borrow().value);
                println!("{:?} <- {:?}\n", update[level].as_ref().unwrap().borrow().value, new_node.borrow().value);
            }
        }

    }

    fn get_mut_recursive(&self, cursor: &Rc<RefCell<Node<T>>>, mut update: Vec<Link<T>>, value: &T, level: usize) -> Vec<Link<T>> {
        if let Some(node) = &cursor.borrow().forward[level] {
            if node.borrow().value < *value {
                return self.get_mut_recursive(node, update, value, level);
            }
        }
        update[level] = Some(Rc::clone(cursor));

        if level > 0 {
            return self.get_mut_recursive(cursor, update, value, level-1);
        }
        update
    }

    pub fn insert(&mut self, value: T) {
        let random_level = self.random_level();
        let new_node = Rc::new(RefCell::new(Node { 
            value: value, 
            forward: vec![None; random_level+1]
        }));

        // skiplist vazia
        if self.head[0].is_none() {
            for level in 0..=random_level {
                if self.head[level].is_none() {
                    self.head[level] = Some(Rc::clone(&new_node));
                }
            }   
        }
        // inserir novo item como head
        else if self.head[0].as_ref().unwrap().borrow().value > new_node.borrow().value {
            let mut inner_value = new_node.take();
            inner_value.forward = self.head[0..=random_level].to_vec();
            new_node.replace(inner_value);

            for level in 0..=random_level {
                self.head[level] = Some(Rc::clone(&new_node));
            }
        }
        // busca e inserção recursiva
        else {
            let mut level = random_level;
            while self.head[level].is_none() {
                self.head[level] = Some(Rc::clone(&new_node));
                level -= 1;
            }
            let cursor = self.head[level].as_ref();
            self.recursive_insert(cursor.unwrap(), &new_node, level);
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