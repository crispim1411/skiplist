use std::fmt::Debug;

#[derive(Debug)]
struct Item<T> {
    value: T,
    next: Link<T>,
}
type Link<T> = Option<Box<Item<T>>>;

pub struct Stack<T> {
    head: Link<T>
}

impl<T: Debug> Stack<T> {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let new_head = Item {
            value,
            next: old_head,
        };
        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take();
        match old_head {
            Some(item) => {
                self.head = item.next;
                Some(item.value)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(item) => Some(&item.as_ref().value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_empty_test() {
        let empty_stack: Stack<u32> = Stack::empty();
        assert_eq!(empty_stack.peek(), None);
    }

    #[test]
    fn push_item_test() {
        let mut empty_stack = Stack::empty();
        empty_stack.push(10);
        assert_eq!(empty_stack.peek(), Some(&10));
    }

    #[test]
    fn pop_item_test() {
        let mut empty_stack = Stack::empty();
        empty_stack.push(20);
        empty_stack.push(30);
        assert_eq!(empty_stack.pop(), Some(30));
    }
}

