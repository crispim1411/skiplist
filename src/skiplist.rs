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
        println!("##Inserting {:?}", value);
        for level in 0..random_level {
            println!("--Leve: {}--", level);
            self.recursive_insert(&self.head[self.level-1], value.clone(), level);
        }
    }

    fn recursive_insert(&self, cursor: &Link<T>, value: T, level: usize) {
        if let Some(node) = cursor {
            if let Some(next_node) = node.borrow().forward[level].as_ref() {
                if next_node.borrow().value < value {
                    return self.recursive_insert(&node.borrow().forward[level], value, level);
                }
            }
            let mut old_value = node.take();
            println!("node: {:?}", old_value);
            let update_ref = old_value.forward;

            let new_node = Node{ value, forward: update_ref};
            let ref_new = 
                Rc::new(RefCell::new(new_node));

            old_value.forward = vec![Some(Rc::clone(&ref_new)); 1];

            *node.borrow_mut() = old_value;
        }   
    }
} 

impl SkipList<u32> {
    pub fn new() -> Self {
        let mut head = vec![None; MAX_LEVEL];

        // node2
        let random_level2 = 2;
        let node2 = Node {
            value: 7,
            forward: vec![None; random_level2]
        };
        let rc_node2 = Rc::new(RefCell::new(node2));
        // for level in 0..random_level2 {
        //     if head[level].is_none() {
        //         println!("inserting {:?} on head[{}]: {:?}",rc_node2, level, head[level]);
        //         head[level] = Some(Rc::clone(&rc_node2));
        //     }
        // }
        
        // node1
        let random_level = 2;
        let mut node1 = Node {
            value: 2,
            forward: vec![None; random_level]
        };
        for level in 0..random_level2 {
            println!("inserting {:?} on node[{}]: {:?}",rc_node2.borrow().value, level, node1.forward[random_level2 - 1] );
            node1.forward[level]  = Some(Rc::clone(&rc_node2));
        }
       // node1.forward[random_level2 - 1] = Some(Rc::clone(&rc_node2));
        let rc_node1 = Rc::new(RefCell::new(node1));
        for level in 0..random_level {
            println!("inserting {:?} on head[{}]: {:?}",rc_node1.borrow().value, level, head[level]);
            head[level] = Some(Rc::clone(&rc_node1));
        }
        println!("node inserted: {:?}", rc_node1);

        
        
        Self { 
            head, 
            level: 1
        }
    }

}