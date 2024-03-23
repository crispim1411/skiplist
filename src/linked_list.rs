use std::fmt::Debug;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> where T: PartialOrd + PartialEq {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn show(&self) where T: Debug {
        match &self.head {
            Some(head) => Self::show_recursive(head),
            None => println!("Linked List is empty")
        }
    }

    fn show_recursive(cursor: &Box<Node<T>>) where T: Debug {
        print!("[{:?}]", cursor.value);
        match &cursor.next {
            Some(next_item) => Self::show_recursive(next_item),
            None => print!("End")
        }
    }

    pub fn insert_with_logs(&mut self, value: T) where T: Debug {
        println!("Inserting {value:?}");
        self.insert(value);
    }

    pub fn insert(&mut self, value: T) {
        let Some(head) =  &mut self.head else {
            let new_item = Node { value, next: None };
            self.head = Some(Box::new(new_item));
            return
        };

        if value < head.value {
            let head_node = std::mem::take(&mut self.head);
            let new_node = Node { value, next: head_node };
            self.head = Some(Box::new(new_node));
        }
        else  {
            LinkedList::recursive_insert(head, value);
        }
    }

    fn recursive_insert(cursor: &mut Box<Node<T>>, value: T) {
        if let Some(next_node) = &mut cursor.next {
            if next_node.value < value {
                return Self::recursive_insert(next_node, value);
            }
            let bigger_node = std::mem::take(&mut cursor.next);
            let new_node = Node { value, next: bigger_node };
            cursor.next = Some(Box::new(new_node));
            return
        }
        let new_node = Node { value, next: None };
        cursor.next = Some(Box::new(new_node));
    }

    pub fn delete(&mut self, value: T) {
        let Some(head) = &mut self.head else {
            println!("Linked List is empty");
            return
        };

        if head.value == value {
            self.head = std::mem::take(&mut head.next);
        } else if head.value < value {
            Self::recursive_delete(head, value)
        } else {
            println!("Value not found");
        }
    }

    fn recursive_delete(cursor: &mut Box<Node<T>>, value: T) {
        if let Some(next_node) = &mut cursor.next {
            if next_node.value < value {
                return Self::recursive_delete(next_node, value);
            } else if next_node.value == value {
                cursor.next = std::mem::take(&mut next_node.next);
                return
            }
        }
        println!("Value not found");
    }

    #[allow(dead_code)]
    fn get_node_ref<'a>(
        &self, 
        cursor: &'a Option<Box<Node<T>>>, 
        key: &T
    ) -> &'a Option<Box<Node<T>>> 
    where T: PartialEq + PartialOrd {
        if let Some(node) = cursor.as_ref() {
            if let Some(next_node) = node.next.as_ref() {
                if next_node.value < *key {
                    return self.get_node_ref(&node.next, key);
                }
            }   
        }
        cursor
    }
}

impl<T: Clone> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.head.take() {
            Some(node) => { 
                self.head = node.next;
                Some(node.value) 
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_ref_test(){
        let mut list = LinkedList::new();
        list.insert(7);
        list.insert(3);
        list.insert(5);
        let ref_node = list.get_node_ref(&list.head, &6);
        assert_eq!(ref_node.as_ref().unwrap().value, 5);
    }

    #[test]
    fn insert_test() {
        let mut list = LinkedList::new();
        list.insert(2);
        list.insert(7);
        list.insert(3);
        list.insert(5);
        assert_eq!(list.head.as_ref().unwrap().value, 2); //head
        let ref_node = list.get_node_ref(&list.head, &10);
        assert_eq!(ref_node.as_ref().unwrap().value, 8); //tail
    }

    #[test]
    fn delete_test() {
        let mut list = LinkedList::new();
        list.insert(2);
        list.insert(7);
        list.insert(3);
        list.insert(5);
        list.delete(2);
        list.delete(5);
        assert_eq!(list.head.as_ref().unwrap().value, 3);
        let ref_node = list.get_node_ref(&list.head, &6);
        assert_eq!(ref_node.as_ref().unwrap().value, 3);
    }

    #[test]
    fn delete_one_item_test() {
        let mut list = LinkedList::new();
        list.insert(324);
        list.delete(324);
        assert!(list.head.is_none());
    }

    #[test]
    fn iter_test() {
        let mut list = LinkedList::new();
        for i in (0..20).rev() {
            list.insert(i);
        }
        let v: Vec<i32> = list.collect();
        println!("List to vector: {:?}", v);
    }
}

