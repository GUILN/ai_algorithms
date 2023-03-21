# Cannibals and Missionaries

[Repositório](https://github.com/GUILN/ai_algorithms)

## Para Executar

1. Faça a instalação padrão do `Rust` (`rustc` + `cargo`), seguindo: https://www.rust-lang.org/tools/install
2. Se for utilizar o [Makefile](./Makefile), execute: `make build`.
2. Se for utilizar diretamente o [cargo](https://doc.rust-lang.org/stable/rust-by-example/cargo.html), execute: `cargo build`.
3. Após realizar o build, siga a tabela para rodar cada algoritmo:

| Algoritmo | Executar diretamente com o cargo | Executar com o [Makefile](./Makefile) |
|-----------|----------------------------------|---------------------------------------|
| BFS | `cargo run --bin bfs` | `make run_bfs` |
| DFS | `cargo run --bin dfs` | `make run_dfs` |
| Best First Search | `cargo run --bin greedy_best_first_search` | `make run_greedy_best_first_search` |
| A* | `cargo run --bin a_star` | `make run_a_star` |

Para Executar todos de uma vez (somente pelo [Makefile](./Makefile)):
```bash
make run_all
``` 

## Para Executar Testes

Para me ajudar durante o desenvolvimento do problema, ao longo da implementação passei a adicionar testes para:
1. Saber que o código estava tendo o comportamento esperado
2. Evitar "quebrar" a solução quando precisava alterar alguma coisa (uma vez que reutilizei para todos os algoritmos as mesmas classes de domínio [nesse módulo](./src/cannibals/))

Os testes estão presentes no final de cada arquivo do [módulo de domínio](./src/cannibals/), e podem ser executados da seguinte maneira:
```bash
cargo test
# ou
make test
``` 

## Detalhes de Implementação 

### Domínio
Como o domínio era comum entre todos os algoritmos, optei por reutilizá-lo. ele pode ser encontrado [nesse módulo](./src/cannibals/) e está separado em:
* `SideState`: O estado de cada lado da margem (quantos canibais, quantos missinários, tem ou não tem barco)
* `WorldState`: O estado do jogo completo composto por 2 `SideState` (right e left).

Esses módulos possuem também os testes e os comportamentos para identificar `gameover state`, `goal state`, gerar estados filhos, e retornar funções de custo e heurística.

### Estruturas de dados do Rust
Para armazenar os próximos estados a serem visitados foram utilizados:

| Algoritmo | Estrutura de dados | Estrutura no Rust |
|-----------|--------------------|-------------------|
| BFS | Fila | `VecDeque<WorldState>` com: `.push_back()` e `.pop_front()` |
| DFS | Pilha | `VecDeque<WorldState>` com: `.push_front()` e `.pop_font()` |
| Algoritmos de Heurística | Min-Heap | `BinaryHeap<Reverse<WorldStateHeapWrapper>>` com: `.push()` e `.pop()` |


Struct `WorldStateHeapWrapper`:
Foi feita para poder utilizar a implementação de `BinaryHeap` das bibliotecas padrão do `Rust`. 

É composta por uma referência a `WorldState`, e `Tipo de heurística` (só heurística, ou heurística + custo de abertura do nó).

Ao ser utilizada no `BinaryHeap` é utilizada dentro da estrutura `Reverse` para obter comportamento de `Min-Heap` (menor custo).
