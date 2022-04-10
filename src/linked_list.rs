use std::fmt::Debug;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> where T: Debug + PartialOrd + Debug {
    pub fn new(value: T) -> Self {
        let node = Node { value, next: None };
        Self {
            head: Some(Box::new(node))
        }
    }

    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn insert(&mut self, value: T) {
        if self.head == None {
            self.head = Some(Box::new(Node { value, next: None }));
        } 
        else if value < self.head.as_ref().unwrap().value {
            let mut new_node = Node { value, next: None };
            let old_value = mem::take(&mut self.head);
            new_node.next = old_value;
            self.head = Some(Box::new(new_node));
        }
        else  {
            LinkedList::recursive_insert(&mut self.head, value);
        }
    }

    fn recursive_insert(cursor: &mut Option<Box<Node<T>>>, value: T) {
        if let Some(node) = cursor {
            if let Some(next_node) = &mut node.next {
                if next_node.value < value {
                    LinkedList::recursive_insert(&mut node.next, value);
                    return
                } 
            } 
            let mut new_node = Node { value, next: None };
            let old_value = mem::take(&mut node.next);
            new_node.next = old_value;
            node.next = Some(Box::new(new_node));
        }
    }

    pub fn display(&self) {
        let mut cursor = &self.head;
        while let Some(node) = cursor {
            println!("{:?}", node.value);
            cursor = &node.next;
        }
    }

    pub fn delete(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_empty_test() {
        let empty_LinkedList: LinkedList<u32> = LinkedList::empty();
        assert_eq!(empty_LinkedList.head, None);
    }
}

