# SkipList
SkipList, é uma estrutura descrita em 1989 por William Pugh que se baseia em balancear de forma probabilística atalhos de um item a outro.O objetivo deste projeto é de implementar uma Skip List em Rust. Dada a complexidade de desenvolvimento dado o gerenciamento de memória, foi decidido a implementação de estruturas intermediárias.

## Plano de desenvolvimento
1. Stack: Entender o processo de substituição de um item por outro na cabeça da pilha
1. Linked List: Inserir um item entre outros dois itens da lista
1. SkipList: Em cada nível inserir entre dois itens um novo item

## Complexidade temporal
### Um nível
Iniciando em uma lista simples ordenada a complexidade temporal média de busca é O(n)

![image](https://user-images.githubusercontent.com/29204714/162274513-f24f34d8-e1b9-4c17-aa2c-b3c687e743ce.png)

### Dois níveis
Vamos então incluir acima uma lista com atalhos a cada dois itens. 

![image](https://user-images.githubusercontent.com/29204714/162274639-8b236949-5c30-40d7-84bb-1aabe8b26837.png)

A complexidade temporal pode ser calculada por: 

![equation](https://latex.codecogs.com/svg.image?\text{Custo&space;temporal}&space;\approx&space;\mid&space;L1&space;\mid&space;&plus;&space;\frac{\mid&space;L0&space;\mid}{\mid&space;L1&space;\mid})

### Generalização
Incluindo mais um nível a complexidade se torna ![equation](https://latex.codecogs.com/svg.image?3\sqrt[3]{n})

![image](https://user-images.githubusercontent.com/29204714/162274763-6ba12002-8007-4b8c-a4df-5ff72551bfc4.png)
