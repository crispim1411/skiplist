use std::fmt::Debug;

#[derive(Debug, PartialEq)]
struct Node<T> {
    value: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>
}

impl<T> LinkedList<T> where T: Debug {
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
        todo!()
    }

    pub fn delete(&mut self) {
        todo!()
    }

    pub fn display(&self) {
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

