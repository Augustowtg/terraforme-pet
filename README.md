# terraforme-pet

Janela flutuante com o mascote do [terraforme](https://github.com/Augustowtg/terraforme)
(o tatu-peba). Fica sobre as outras janelas, mostra pelo humor do mascote o que
os agentes de todos os `terraforme cli` estão fazendo, avisa (balão +
`notify-send` no Linux, notificação do sistema no Windows) quando um termina
ou espera resposta e, com um clique, abre o menu dos agentes para ir a um
deles.

Linux (X11 e Wayland) e Windows 10/11. No Windows, escolher um agente traz
para frente a janela do terminal dele (Windows Terminal, console clássico ou
o terminal do VS Code).

## Instalação

Como plugin do terraforme (≥ 0.25.0; no Windows, ≥ 0.26.0):

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
`x86_64-unknown-linux-gnu` e `aarch64-unknown-linux-gnu`, e
`terraforme-pet-x86_64-pc-windows-msvc.zip` + `.sha256`. O contrato com o
instalador do terraforme está em `terraforme-plugin.toml`.

Para compilar contra um `terraforme-core` local (antes de ele chegar na `main`):

```bash
cargo build --config 'patch."https://github.com/Augustowtg/terraforme.git".terraforme-core.path="../terraforme/crates/terraforme-core"'
```
