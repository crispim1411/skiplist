use rand::Rng;
use skiplist::stack::Stack;
use skiplist::linked_list::LinkedList;

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

fn run_linked_list() {
    println!("*Geração de números aleatórios*");
    let mut list = LinkedList::empty();
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let random: f32 = rng.gen_range(0.0..100.0);
        println!("Inserting: {}", random);
        list.insert(random);
    }
    println!("--------------------\nFim");
    list.display();
}

fn main() {
    run_linked_list();
}