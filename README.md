# terraforme-pet

Janela flutuante com o mascote do [terraforme](https://github.com/Augustowtg/terraforme)
(o tatu-peba). Fica sobre as outras janelas, mostra pelo humor do mascote o que
os agentes de todos os `terraforme cli` estão fazendo, avisa (balão +
`notify-send`) quando um termina ou espera resposta e, com um clique, abre o
menu dos agentes para ir a um deles.

Só Linux por enquanto (X11 e Wayland).

## Instalação

Como plugin do terraforme (≥ 0.24.0):

```bash
terraforme plugin install pet
```

Ou compilando com Rust:

```bash
cargo install --locked --git https://github.com/Augustowtg/terraforme-pet terraforme-pet
```

Depois, `terraforme pet` abre a janela (ou `terraforme config --pet true` para
abri-la junto com o `terraforme cli`).

## Desenvolvimento

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

Uma tag `v*` (igual à versão do `Cargo.toml`) dispara o `release.yml`, que
publica `terraforme-pet-<target>.tar.gz` + `.sha256` para
`x86_64-unknown-linux-gnu` e `aarch64-unknown-linux-gnu`. O contrato com o
instalador do terraforme está em `terraforme-plugin.toml`.
