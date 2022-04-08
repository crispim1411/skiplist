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

    pub fn print(&self) {
        match &self.head {
            Some(_) => Stack::print_rec(self.head.as_ref()),
            None => println!("Empty stack"),
        }
    }

    fn print_rec(mut cursor: Option<&Box<Item<T>>>) {
        while let Some(item) = cursor {
            print!("{:?} ", item.value);
            cursor = item.next.as_ref();
        }
        println!();
    }
}
