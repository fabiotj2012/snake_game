
# Tutorial Snake Game com Rust e WebAssembly

Este tutorial irá guiá-lo através da estrutura e funcionamento do jogo Snake, desenvolvido com Rust e WebAssembly.

## Pré-requisitos

- **Rust:** Instale via [rustup](https://rustup.rs/).
- **wasm-pack:** Instale com `cargo install wasm-pack`.

## Visão Geral do Projeto e Estrutura de Arquivos

O projeto está estruturado para ser executado tanto como uma aplicação nativa quanto em um navegador web usando WebAssembly.

### Pasta `src` - Código Fonte

- **`src/lib.rs`**: O coração da sua biblioteca Rust. Contém a lógica principal do jogo, que é compartilhada entre o executável nativo e o WebAssembly. A compilação condicional (`#[cfg(...)]`) garante que o código específico do `wasm-bindgen` seja usado apenas no alvo web.
- **`src/main.rs`**: O ponto de entrada para a compilação nativa (EXE). Ele atua como um programa Rust padrão que importa e utiliza a funcionalidade da sua biblioteca (`lib.rs`).
- **`src/index.html`**: A página web que o usuário final abrirá no navegador. Sua única função é carregar o script JavaScript que inicializa o WASM.
- **`src/index.js`**: O "lançador" do seu WebAssembly. Ele importa o módulo WASM, o inicializa e chama as funções exportadas do Rust. Ele também gerencia a memória e outras interações complexas.
- **`src/bootstrap.js`**: Este arquivo é responsável por importar o `index.js` e lidar com erros de importação.

### Pasta `pkg` - Pacote WebAssembly

Esta pasta é **gerada automaticamente** pelo `wasm-pack`. Ela contém tudo o que é necessário para usar seu código Rust no navegador.

- **`snake_game_bg.wasm`**: O seu código Rust compilado para o formato binário WebAssembly. Este é o arquivo que o navegador realmente executa.
- **`snake_game.js`**: O "cola" JavaScript gerado pelo `wasm-bindgen`. Ele cria uma ponte entre o JavaScript e o seu código WASM, permitindo que você chame funções Rust diretamente do seu código JS.
- **`snake_game.d.ts`**: Arquivo de definição de tipos para TypeScript. Fornece autocompletar e verificação de tipos se você estiver usando TypeScript no seu projeto web.
- **`package.json`**: Um arquivo de manifesto que descreve este pacote, listando os arquivos que o compõem. É útil para a integração com ecossistemas JavaScript modernos.

## Lógica do Jogo (`src/lib.rs`)

O arquivo `src/lib.rs` é o coração do jogo. Ele define as principais estruturas de dados e a lógica do jogo.

### Estruturas de Dados

- **`Direction`**: Enum que representa as direções possíveis da cobra (Cima, Baixo, Esquerda, Direita).
- **`Point`**: Struct que representa um ponto no grid do jogo, com coordenadas `x` e `y`.
- **`Snake`**: Struct que representa a cobra, contendo seu corpo (um vetor de `Point`) e a direção atual.
- **`Game`**: Struct principal que contém o estado do jogo, incluindo a largura e altura do grid, a cobra, a comida, a pontuação e o estado de "game over".

### Lógica Principal

- **`Game::new()`**: Cria uma nova instância do jogo.
- **`Game::tick()`**: Avança o estado do jogo em um "tick" de tempo. Move a cobra, verifica colisões com as paredes e com o próprio corpo, e verifica se a cobra comeu a comida.
- **`Game::change_snake_direction()`**: Altera a direção da cobra.
- **`Game::spawn_food()`**: Gera uma nova comida em uma posição aleatória no grid.

## Versão WebAssembly

A versão WebAssembly é compilada a partir do `src/lib.rs` e é exposta para o JavaScript através do `wasm-bindgen`.

### `#[cfg(target_arch = "wasm32")]`

Este atributo de compilação condicional garante que o código dentro do bloco `mod wasm` só seja compilado quando o alvo é `wasm32`.

### `WasmGame`

Dentro do módulo `wasm`, a struct `WasmGame` atua como um wrapper em torno da struct `Game` principal. Isso permite expor uma API mais amigável para o JavaScript.

### `src/index.js`

Este arquivo é responsável por:

1.  **Inicializar o WebAssembly**: Carrega o módulo `snake_game.js` gerado.
2.  **Configurar o Canvas**: Obtém o elemento canvas do HTML e define suas dimensões.
3.  **Criar a Instância do Jogo**: Cria uma nova instância de `Game` a partir do módulo WebAssembly.
4.  **Lidar com a Entrada do Usuário**: Adiciona um `event listener` para as teclas de seta para controlar a cobra.
5.  **Loop do Jogo**: A função `gameLoop` é chamada repetidamente para atualizar e desenhar o estado do jogo no canvas.

## Versão Nativa (`src/main.rs`)

A versão nativa utiliza a biblioteca `ggez` para criar uma janela e desenhar o jogo.

### `#[cfg(not(target_arch = "wasm32"))]`

Este atributo garante que o código em `src/main.rs` não seja compilado para o alvo WebAssembly.

### `AppState`

A struct `AppState` armazena o estado da aplicação para o `ggez`, que neste caso é a instância do `Game`.

### `EventHandler`

A implementação do `EventHandler` para o `AppState` define como o jogo responde a eventos como atualizações de frame (`update`) e renderização (`draw`), bem como a entrada do teclado (`key_down_event`).

## Como Executar

### Versão Web

1.  Certifique-se de ter o `wasm-pack` instalado.
2.  Compile o projeto para WebAssembly: `wasm-pack build --target web`
3.  Inicie um servidor web local no diretório raiz do projeto (por exemplo, com `npm start`).
4.  Abra o navegador em `http://localhost:8080` (ou a porta que seu servidor estiver usando).

### Versão Nativa

1.  Execute o projeto com o Cargo: `cargo run`

