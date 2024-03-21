# InfiniteCraft Scraper

Este é um programa em Rust para fazer scraping do jogo InfiniteCraft. Ele usa a biblioteca Isahc para fazer requisições HTTP, Tokio para execução de requisições assíncronas e rusqlite para salvar as combinações em um banco de dados SQLite.

## Visão Geral do Jogo

Infinite Craft é um jogo de criação baseado em lógica que desafia os jogadores a combinarem uma variedade máxima de itens usando ingredientes básicos. Os jogadores começam com apenas Água, Fogo, Vento e Terra, e têm a liberdade de combinar esses elementos simplesmente clicando e arrastando-os no espaço de jogo. Além disso, é possível unir dois itens idênticos para criar algo novo, como combinar duas árvores para formar uma floresta.

## Funcionalidades do Programa

- [x] Scraping de Combinacões: O programa é capaz de extrair as possíveis combinações de elementos disponíveis no jogo InfiniteCraft e salvá-las em um banco de dados SQLite.

- [ ] Geração de Árvore de Combinacões: Pretende-se implementar uma funcionalidade para gerar uma árvore de combinações para um determinado elemento. Isso permitirá visualizar todas as combinações possíveis que levam a esse elemento específico, proporcionando uma compreensão mais profunda das interações entre os elementos do jogo.

## Bibliotecas Utilizadas

### Isahc

[Isahc](https://crates.io/crates/isahc) é uma biblioteca HTTP para Rust, fornecendo uma API amigável e uma implementação eficiente para fazer requisições e lidar com respostas HTTP.

### Tokio

[Tokio](https://tokio.rs/) é um framework para escrever código assíncrono em Rust. Ele fornece uma runtime para execução de tarefas em paralelo, permitindo que operações de I/O e computacionais sejam realizadas de forma assíncrona e eficiente.

### rusqlite

[rusqlite](https://crates.io/crates/rusqlite) é uma biblioteca que fornece uma interface para interagir com o SQLite em Rust. Ela permite a execução de consultas SQL e a manipulação de bancos de dados SQLite de forma simples e segura.

## Requisitos

Certifique-se de ter o Rust e o Cargo instalados. Você pode instalá-los seguindo as instruções em [Rust Install](https://www.rust-lang.org/learn/get-started).

## Instalação

Clone o repositório:

```
git clone https://github.com/seu-usuario/infinitecraft-scraper.git
```

## Uso

1. Navegue até o diretório do projeto:

```
cd infinitecraft-scraper
```

2. Compile o projeto:

```
cargo build --release
```

3. Execute o programa:

```
cargo run --release
```
