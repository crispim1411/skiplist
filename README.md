# SkipList
SkipList é uma estrutura descrita em 1989 por William Pugh que se baseia em balancear de forma probabilística atalhos de um item a outro com objetivo de alcançar complexidade O(log(n)), sendo assim um substituto para árvores AVL. O objetivo deste projeto é de implementar uma Skip List em Rust, e dada a complexidade de desenvolvimento devido o gerenciamento de memória, foi decidido a implementação de estruturas intermediárias.

## Plano de desenvolvimento
1. Stack: Entender o processo de substituição de um item por outro na cabeça da pilha ([link](https://github.com/crispim1411/skiplist/blob/master/src/stack.rs))
3. Linked List: Inserir um item entre outros dois itens da lista ([link](https://github.com/crispim1411/skiplist/blob/master/src/linked_list.rs))
4. SkipList: Em cada nível inserir entre dois itens um novo item ([link](https://github.com/crispim1411/skiplist/blob/master/src/skiplist.rs))

## Complexidade temporal
### Um nível
Iniciando em uma lista simples ordenada a complexidade temporal média de busca é O(n)

![image](https://user-images.githubusercontent.com/29204714/162274513-f24f34d8-e1b9-4c17-aa2c-b3c687e743ce.png)

### Dois níveis
Vamos então incluir acima uma lista com atalhos a cada dois itens. 

![image](https://user-images.githubusercontent.com/29204714/162274639-8b236949-5c30-40d7-84bb-1aabe8b26837.png)

A complexidade temporal pode ser calculada por: 

![equation](https://latex.codecogs.com/png.image?\dpi{110}\bg{white}\text{Custo&space;temporal}&space;\approx&space;\mid&space;L1&space;\mid&space;&plus;&space;\frac{\mid&space;L0&space;\mid}{\mid&space;L1&space;\mid}&space;)

E para ![equation](https://latex.codecogs.com/png.image?\dpi{110}\bg{white}\mid&space;L1&space;\mid&space;^{2}&space;=&space;\mid&space;L0&space;\mid&space;=&space;n) temos que

![equation](https://latex.codecogs.com/png.image?\dpi{110}\bg{white}\sqrt{n}&space;&plus;&space;\frac{n}{\sqrt{n}}&space;=&space;\frac{2n\sqrt{n}}{n}&space;=&space;2\sqrt{n}&space;)

![equation](https://latex.codecogs.com/png.image?\dpi{110}\bg{white}\text{Custo&space;temporal}&space;=&space;O(\sqrt{n}))

### Generalização
Incluindo mais um nível a complexidade se torna ![equation](https://latex.codecogs.com/png.image?\dpi{110}\bg{white}O(\sqrt[3]{n}))

![image](https://user-images.githubusercontent.com/29204714/162274763-6ba12002-8007-4b8c-a4df-5ff72551bfc4.png)

Para k níveis temos uma complexidade ![equation](https://latex.codecogs.com/png.image?\dpi{110}\bg{white}O(\sqrt[k]{n})). O que aconteceria se tivessemos log(n) níveis? O resultado final, incrivelmente, é O(log(n))

## Randomização da estrutura
Manter uma estrutura onde o nível acima é sempre a metade do nível anterior é muito custoso. Como solução temos a randomização dos níveis. Quando um novo nó é inserido na skiplist seu nível é randomizado, tendo uma probabilidade P de obter uma promoção de nível e probabilidade P^k de obter o nível máximo.

![image](https://user-images.githubusercontent.com/29204714/162480390-97915e50-8ef5-49be-b133-d31f2dc766ed.png)

# Implementação
## Stack
### Inserção
1. Toma o conteúdo do head da stack, deixando valor default
1. Cria um novo item dando o head como próximo 
1. Insere o item como novo head
```rust
pub fn push(&mut self, value: T) {
    let old_head = self.head.take();
    let new_head = Item {
        value,
        next: old_head,
    };
    self.head = Some(Box::new(new_head));
}
```

### Pop
1. Toma o conteúdo do head da stack, deixando valor default
1. Verifica se há conteúdo (Option)
1. Retorna conteúdo ou vazio
```rust
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
```

### Peek
1. Verifica o head da stack (Option)
1. Retorna referência ou vazio

```rust
pub fn peek(&self) -> Option<&T> {
    match &self.head {
        Some(item) => Some(&item.as_ref().value),
        None => None,
    }
}
```
---
## Linked List
### Busca
1. Se o valor do próximo do cursor for menor
    1. próximo vira cursor
1. Senão retorna referência do cursor
```rust
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
```

### Inserção
1. Se o valor do próximo do cursor for menor
    1. Próximo vira cursor
1. Senão toma o valor do próximo ao cursor
1. O novo item recebe o valor tomado como próximo
1. Se torna o novo próximo do cursor
```rust
fn recursive_insert(cursor: &mut Link<T>, value: T) {
    if let Some(node) = cursor {
        if let Some(next_node) = &mut node.next {
            if next_node.value < value {
                return LinkedList::recursive_insert(&mut node.next, value);
            } 
        } 
        let mut new_node = Node { value, next: None };
        let old_value = node.next.take();
        new_node.next = old_value;
        node.next = Some(Box::new(new_node));
    }
}
```

### Remoção
1. Se o valor do próximo do cursor for igual
    1. Toma o valor do próximo
    1. Redireciona o próximo do removido para ser o próximo do cursor
1. Senão se o valor do próximo for menor
    1. Próximo vira cursor
```rust
fn recursive_delete(cursor: &mut Link<T>, value: T) {
    if let Some(node) = cursor {
        if let Some(next_node) = &mut node.next {
            if next_node.value == value {
                let old_value = node.next.take();
                node.next = old_value.unwrap().next;
            }
            else if next_node.value < value {
                LinkedList::recursive_delete(&mut node.next, value);
            }
        }
    }
}
```
---
## SkipList
### Vetor de update 
1. Se o valor do próximo do cursor no nível for menor
    1. O próximo vira cursor
1. Senão preenche o vetor na posição do nível
1. Se nível maior que zero
    1. Repete para o nível-1
1. Senão retorna o vetor
```rust
fn fill_update_vector(&self, 
    cursor: &Link<T,U>, 
    mut update: Vec<Option<Link<T,U>>>, 
    key: &T, 
    level: usize) -> Vec<Option<Link<T,U>>> {

    if let Some(node) = &cursor.borrow().forward[level] {
        if node.borrow().key < *key {
            return self.fill_update_vector(node, update, key, level);
        }
    }

    update[level] = Some(Rc::clone(cursor));

    if level > 0 {
        return self.fill_update_vector(cursor, update, key, level-1);
    }
    update
}
```

### Inserção
1. Calcula nível randômico para o item
1. Preenche vetor de update
1. Se o valor do próximo ao update for igual ao dado
    1. Item já cadastrado, retorna
1. Senão do nível zero até o nível do novo item
    1. Toma o valor do vetor update[nível]
    1. Toma o valor do novo item 
    1. Próximo do novo item no nível será o próximo do vetor[nível]
    1. Replace do novo item tomado
    1. Próximo do item update[nível] será o novo item
    1. Replace do item update[nível] tomado
1. Se o nível do novo item for maior que o nível atual da estrutura
    1. Estrutura recebe nível
```rust
pub fn insert(&mut self, key: T, value: U) {
    let random_level = self.random_level();
    
    let update = self.fill_update_vector(&self.head, vec![None; random_level+1], &key, random_level);

    if let Some(node) = &update[0] {
        if let Some(next_node) = node.borrow().forward[0].as_ref() {
            if next_node.borrow().key == key {
                println!("Item {:?} already inserted", key);
                return
            }
        }
    }

    let new_node = Rc::new(RefCell::new(
        Node { 
            key, 
            value,
            forward: vec![None; random_level+1]
        }
    ));
    
    for level in 0..=random_level {
        let node = update[level].as_ref().unwrap();

        let mut new_node_inner = new_node.take();
        let mut node_inner = node.take();
        
        new_node_inner.forward[level] = node_inner.forward[level].take();
        new_node.replace(new_node_inner);

        node_inner.forward[level] = Some(Rc::clone(&new_node));
        node.replace(node_inner);
    }

    if random_level > self.level {
        self.level = random_level
    }
}
```

### Remoção
1. Preenche vetor de update
1. Se o valor do próximo ao update for igual ao dado
    1. Preenche item alvo
1. Se item alvo preenchido
    1. Do nível zero até o nível da estrutura
        1. Se próximo de update[nível] for nulo
            1. Retorna
        1. Se próximo de update[nível] não apontar para o item alvo
            1. Retorna
        1. Toma o valor do item alvo 
        1. Toma valor de update[nível]
        1. Redireciona o próximo[nível] do removido para ser o próximo de update[nível]
        1. Replace do item update[nível] tomado
        1. Replace do item removido
1. Senão item não consta na estrutura
 
```rust
pub fn delete(&mut self, key: T) {
    let update = self.fill_update_vector(&self.head, vec![None; self.level+1], &key, self.level);

    let mut option_delete = None;
    if let Some(update_node) = update[0].as_ref() {
        if let Some(next_node) = update_node.borrow().forward[0].as_ref() {
            if next_node.borrow().key == key {
                option_delete = Some(Rc::clone(next_node));
            }
        }
    }
    
    if let Some(delete_node) = option_delete {
        for level in 0..=self.level {                

            let cursor = update[level].as_ref().unwrap();
            if cursor.borrow().forward[level].is_none() {
                break;
            }
            else if !Rc::ptr_eq(
                cursor.borrow().forward[level].as_ref().unwrap(), 
                &delete_node) {
                break;
            }

            let mut delete_node_inner = delete_node.take();
            let mut node_inner = cursor.take();

            node_inner.forward[level] = delete_node_inner.forward[level].take();

            cursor.replace(node_inner);
            delete_node.replace(delete_node_inner);
        }
        while self.level > 1 && self.head.borrow().forward[self.level].is_none() {
            self.level -= 1;
        }
    }
    else {
        println!("Item {:?} not found", key);
    }
}
```
---
## Considerações Finais
- O dado da SkipList foi englobado numa estrutura Rc para permitir múltiplas referências ao objeto 
- Cogitou-se utilizar internamente uma estrutura Cell porém não se mostrou possível implementar Copy para cumprir o método Get da estrutura
- A Reescrita de ponteiros foi feita através do esquema Take-Change-Replace
- Esse repositório se destina ao aprendizado, logo quaisquer críticas ou dicas são bem vindas!



