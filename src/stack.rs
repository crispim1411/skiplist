#[derive(Debug)]
struct Item {
    value: u32,
    next: Link,
}
type Link = Option<Box<Item>>;

pub struct Stack {
    head: Link
}

impl Stack {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: u32) {
        let old_head = self.head.take();
        let new_head = Item {
            value,
            next: old_head,
        };
        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<u32> {
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

    fn print_rec(mut cursor: Option<&Box<Item>>) {
        while let Some(item) = cursor {
            print!("{:?} ", item.value);
            cursor = item.next.as_ref();
        }
        println!();
    }
}
