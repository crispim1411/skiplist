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
    println!("*Campeonato*");
    let mut list = LinkedList::empty();
    list.insert(&2.8);
    list.insert(&7.5);
    list.insert(&1.7);
    list.insert(&8.1);
    list.insert(&1.3);
    list.insert(&6.9);
    list.insert(&0.0);
    println!("--------------------\nEncerramento");
    println!("--------------------\nPódio");
    list.display();
}

fn main() {
    run_linked_list();
}