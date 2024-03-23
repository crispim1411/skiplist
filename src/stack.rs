use std::fmt::Debug;

struct Item<T> {
    value: T,
    next: Option<Box<Item<T>>>,
}

pub struct Stack<T> {
    top: Option<Box<Item<T>>>
}

impl<T> Stack<T> {
    pub fn empty() -> Self {
        Stack { top: None }
    }

    pub fn push(&mut self, value: T) {
        let top = std::mem::take(&mut self.top);
        let new_top = Item {
            value,
            next: top
        };
        self.top = Some(Box::new(new_top))
    }

    pub fn push_with_log(&mut self, value: T) where T: Debug {
        println!("Inserting {value:?}");
        self.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        let top = std::mem::take(&mut self.top);
        match top {
            Some(item) => {
                self.top = item.next;
                return Some(item.value);
            },
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(item) => Some(&item.value),
            None => None,
        }
    }

    pub fn show(&self) where T: Debug {
        match &self.top {
            Some(item) => Self::show_recursive(item),
            None => println!("Stack is empty")
        }
    }

    fn show_recursive(cursor: &Box<Item<T>>) where T: Debug {
        print!("[{:?}] -> ", cursor.value);
        match &cursor.next {
            Some(next_item) => Self::show_recursive(next_item),
            None => print!("End")
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

