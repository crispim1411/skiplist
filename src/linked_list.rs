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

    pub fn display(&self) {
        let mut cursor = &self.head;
        while let Some(node) = cursor {
            println!("{:?}", node.value);
            cursor = &node.next;
        }
    }

    pub fn insert(&mut self, value: T) {
        // Lista vazia
        if self.head == None {
            self.head = Some(Box::new(Node { value, next: None }));
        } 
        // Inserir um novo item como head
        else if value < self.head.as_ref().unwrap().value {
            let mut new_node = Node { value, next: None };
            let old_value = mem::take(&mut self.head);
            new_node.next = old_value;
            self.head = Some(Box::new(new_node));
        }
        // Busca e inserção
        else  {
            LinkedList::recursive_insert(&mut self.head, value);
        }
    }

    fn recursive_insert(cursor: &mut Link<T>, value: T) {
        if let Some(node) = cursor {
            if let Some(next_node) = &mut node.next {
                if next_node.value < value {
                    return LinkedList::recursive_insert(&mut node.next, value);
                } 
            } 
            let mut new_node = Node { value, next: None };
            let old_value = mem::take(&mut node.next);
            new_node.next = old_value;
            node.next = Some(Box::new(new_node));
        }
    }

    pub fn delete(&mut self, value: T) {
        if let Some(head_node) = &mut self.head{
            if head_node.value == value {
                let old_value = mem::take(&mut self.head);
                self.head = old_value.unwrap().next;
            } else {
                LinkedList::recursive_delete(&mut self.head, value);
            }
        } else {
            println!("Empty list");
        }
    }

    fn recursive_delete(cursor: &mut Link<T>, value: T) {
        if let Some(node) = cursor {
            if let Some(next_node) = &mut node.next {
                if next_node.value == value {
                    let old_value = mem::take(&mut node.next);
                    node.next = old_value.unwrap().next;
                }
                else if next_node.value < value {
                    LinkedList::recursive_delete(&mut node.next, value);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn get_node_ref<'a>(&self, cursor: &'a Link<T>, key: &T) -> &'a Link<T> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_empty_test() {
        let empty_list: LinkedList<u32> = LinkedList::empty();
        assert_eq!(empty_list.head, None);
    }

    #[test]
    fn get_ref_test(){
        let mut list = LinkedList::new(2);
        list.insert(7);
        list.insert(3);
        list.insert(5);
        let ref_node = list.get_node_ref(&list.head, &6);
        assert_eq!(ref_node.as_ref().unwrap().value, 5);
    }

    #[test]
    fn insert_test() {
        let mut list = LinkedList::new(8);
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
        let mut list = LinkedList::new(8);
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
        let mut list = LinkedList::empty();
        list.insert(324);
        list.delete(324);
        assert_eq!(list.head, None);
    }
}

