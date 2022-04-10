use std::{fmt::Debug, mem, ops::DerefMut};

#[derive(Debug, PartialEq)]
struct Node<'a,T> {
    value: &'a T,
    next: Link<'a,T>,
}
type Link<'a,T> = Option<Box<Node<'a,T>>>;

pub struct LinkedList<'a,T> {
    head: Link<'a,T>
}

impl<'a,T> LinkedList<'a,T> where T: Debug + PartialOrd + Debug {
    pub fn new(value: &'a T) -> Self {
        let node = Node { value, next: None };
        Self {
            head: Some(Box::new(node))
        }
    }

    pub fn empty() -> Self {
        Self { head: None }
    }

    

    pub fn insert(&mut self, value: &'a T) {
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
            recursive_insert(&mut self.head, &value);
        }
    }

    pub fn delete(&mut self) {
        todo!()
    }

    pub fn display(&self) {
        let mut cursor = &self.head;
        while let Some(node) = cursor {
            println!("{:?}", node.value);
            cursor = &node.next;
        }
    }
}

fn recursive_insert<'a,T: Debug + PartialOrd>(cursor: &mut Option<Box<Node<'a,T>>>, value: &'a T) {
    if let Some(node) = cursor {
        if let Some(next_node) = &mut node.next {
            if next_node.value < value {
                recursive_insert(&mut node.next, value);
                return
            } 
        } 
        let mut new_node = Node { value, next: None };
        let old_value = mem::take(&mut node.next);
        new_node.next = old_value;
        node.next = Some(Box::new(new_node));
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

