use rand::Rng;
use skiplist::stack::Stack;
use skiplist::skiplist::SkipList;
use skiplist::linked_list::LinkedList;

#[allow(dead_code)]
fn run_stack() {
    println!("*Viagem*");
    let mut stack = Stack::empty();
    println!("--------------------\nArrumando a mala");
    stack.push(String::from("Carteira"));
    stack.push(String::from("Calça jeans"));
    stack.push(String::from("Calça jeans velha"));
    stack.push(String::from("Suéter"));
    stack.push(String::from("Par de meias"));
    stack.push(String::from("Cueca"));
    stack.push(String::from("Creatina"));
    stack.push(String::from("Notebook"));
    stack.push(String::from("Carregador"));
    println!("--------------------\nAbre a mala");
    match stack.peek() {
        Some(value) => println!("{:?}", value),
        None => println!("Mala está vazia"),
    }
    println!("--------------------\nDesarrumando a mala");
    while let Some(item) = stack.pop() {
        println!("tira {:?}", item);
    }
}

#[allow(dead_code)]
fn run_random_linked_list() {
    println!("*Geração de números aleatórios*");
    let mut list = LinkedList::empty();
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let random: f32 = rng.gen_range(0.0..100.0);
        println!("Inserting: {}", random);
        list.insert(random);
    }
    println!("Fim--------------------");
    let v: Vec<f32> = list.collect();
    println!("Ordered vector: {:?}", v);
}

fn run_odd_linked_list() {
    let last = 20;
    println!("*Inserindo números de 0 a {}*", last);
    let mut list = LinkedList::empty();
    for i in 0..=last {
        list.insert(i);
    }
    for j in (0..=last).step_by(2) {
        println!("Removing {}", j);
        list.delete(j);
    }
    let v: Vec<i32> = list.collect();
    println!("Odd list: {:?}", v);
}

fn run_skiplist() {
    let mut skiplist: SkipList<u32> = SkipList::new();
    println!("SL: {:?}", skiplist);
    skiplist.insert(5);
    skiplist.insert(3);

    skiplist.display();
}

fn main() {
    // run_stack();
    // println!("\n");
    // run_random_linked_list();
    // println!("\n");
    // run_odd_linked_list();
    run_skiplist();
}