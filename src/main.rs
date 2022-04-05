use std::rc::Rc;
use rand::Rng;

const P: f32 = 0.5;

#[derive(Debug)]
struct Node<T> {
    key: isize,
    value: Option<T>,
    forward: Vec<NodeRef<T>>,
}
type NodeRef<T> = Option<Rc<Node<T>>>;

impl<T> Node<T> {
    fn new(key: isize, value: Option<T>, level: usize) -> Node<T> {
        Node {
            key,
            value,
            forward: vec![None; level],
        }
    }
}

#[derive(Debug)]
struct SkipList {
    head: NodeRef<u32>,
    level: usize,
    maxLevel: usize,
    p: f32,
}

impl SkipList {
    fn new(maxLevel: usize) -> SkipList {
        let head = Rc::new(Node::new( -1, None, maxLevel));
        SkipList {
            head: Some(head),
            level: 0,
            maxLevel,
            p: P
        }
    }

    fn insert(&mut self, key: isize, value: Option<u32>) {
        let mut update = vec![&None; self.maxLevel];
        let mut current = &self.head;

        for i in (1..=self.level).rev() {
            while let Some(node) = current {
                match &node.forward[i] {
                    Some(fw) => {
                        if fw.key >= key {
                            break;
                        }
                        current = &node.forward[i];
                    }
                    None => break
                }
            }
            update[i] = &current;
        }
        current = &current.as_ref().unwrap().forward[0];
        println!("{:?}", update);
        // UPDATE 
        // if let Some(fw) = current.forward[0].as_ref() {
        //     if fw.key == key {
        //         fw.borrow().value.as_ref().unwrap() = value;
        //     return
        // }

        let random_level = self.random();
        if random_level > self.level {
            for i in (self.level+1)..(random_level+1) {
                update[i] = &self.head;
            }
            self.level = random_level;
        }
        /*
        new_node = Node(key, random_level)
        for i in range(random_level+1):
            new_node.forward[i] = update[i].forward[i]
            update[i].forward[i] = new_node

        */
        let mut new_node = Node::new(key, Some(100), random_level);

        // // Fix
        // for i in 1..random_level+1 {
        //     if let Some(update_node) = update[i] {
        //         if let Some(fw) = &update_node.forward[i] {
        //             new_node.forward[i] = Some(Rc::clone(&fw));     
        //             let rc = Rc::new(new_node);
        //             update_node.forward[i] = Some(Rc::clone(&rc).to_owned());       
        //         }
        //     } 
        // }
    }

    fn random(&self) -> usize {
        let mut level: usize = 1;
        let mut rng = rand::thread_rng();

        while rng.gen::<f32>() < self.p && level < self.maxLevel {
            level += 1;
        }
        return level
    }
}

fn main() {
    let mut sl= SkipList::new(3);
    sl.insert(2, Some(4));
    println!("{:?}", sl);
}
