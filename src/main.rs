use skiplist::stack::Stack;

fn main() {
    println!("Criação de uma stack vazia");
    let mut stack = Stack::empty();
    println!("--------------------\nPush");
    for i in 1..12 {
        stack.push(i);
    }
    stack.print();
    println!("--------------------\nPop");
    for j in 1..7 {
        stack.pop();
    }
    stack.print();
}