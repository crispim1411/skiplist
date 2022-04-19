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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn run_skiplist() {
    let mut skiplist: SkipList<u32, u32> = SkipList::new();
    let mut rng = rand::thread_rng();
    for _ in 0..18 {
        let random: u32 = rng.gen_range(0..20);
        skiplist.insert(random, random*random);
        skiplist.display();
    }   
}

fn run_str_skiplist(){
    #[derive(Debug, Default)]
    struct StudentInfo {
        name: String,
        mat: String,
        grades: Vec<(String, f32)>
    }
    let std1 = StudentInfo {
        name: "Carlos".to_string(),
        mat: "1520030".to_string(),
        grades: vec![
            ("Cálculo 1".to_string(), 5.3),
            ("Álgebra Linear 1".to_string(), 6.6)
        ]
    };
    let std2 = StudentInfo {
        name: "Joana".to_string(),
        mat: "1540020".to_string(),
        grades: vec![
            ("Cálculo 1".to_string(), 7.4),
            ("Álgebra Linear 1".to_string(), 6.8)
        ]
    };
    let std3 = StudentInfo {
        name: "Marcos".to_string(),
        mat: "1560010".to_string(),
        grades: vec![
            ("Cálculo 1".to_string(), 8.4),
            ("Álgebra Linear 1".to_string(), 3.5)
        ]
    };
    let std4 = StudentInfo {
        name: "Hugo".to_string(),
        mat: "1510090".to_string(),
        grades: vec![
            ("Cálculo 1".to_string(), 6.7),
            ("Álgebra Linear 1".to_string(), 6.4)
        ]
    };
    let mut skiplist= SkipList::new();
    skiplist.insert(std1.mat.clone(), std1);
    skiplist.insert(std2.mat.clone(), std2);
    skiplist.insert(std3.mat.clone(), std3);
    skiplist.insert(std4.mat.clone(), std4);
    skiplist.display();
}

fn main() {
    // run_stack();
    // println!("\n");
    // run_random_linked_list();
    // println!("\n");
    // run_odd_linked_list();
    run_str_skiplist();
}