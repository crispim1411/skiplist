# SkipList
SkipList é uma estrutura descrita em 1989 por William Pugh que se baseia em balancear de forma probabilística atalhos de um item a outro com objetivo de alcançar complexidade O(log(n)), sendo assim um substituto para árvores AVL. O objetivo deste projeto é de implementar uma Skip List em Rust, e dada a complexidade de desenvolvimento devido o gerenciamento de memória, foi decidido a implementação de estruturas intermediárias.

## Plano de desenvolvimento
1. Stack: Entender o processo de substituição de um item por outro na cabeça da pilha ([link](https://github.com/crispim1411/skiplist/blob/master/src/stack.rs))
3. Linked List: Inserir um item entre outros dois itens da lista ([link](https://github.com/crispim1411/skiplist/blob/master/src/linked_list.rs))
4. SkipList: Em cada nível inserir entre dois itens um novo item

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
- Inserir na pilha
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
- Remover da pilha 
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

## Linked List
- Pesquisa recursiva retornando referência de nó
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

- Inserir um novo nó recursivamente
```rust
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
```

- Remover um nó recursivamente
```rust
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
```