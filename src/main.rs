//! `terraforme-pet` — janela flutuante com o mascote do terraforme (o
//! tatu-peba). Fica sobre as outras janelas, mostra pelo mascote o que os
//! agentes de todos os `terraforme cli` estão fazendo, avisa quando um termina
//! e, com um clique, abre o menu dos agentes para ir a um deles. Aberta por
//! `terraforme pet` (ou `[tui] pet`). No Linux e no Windows: o macOS está no
//! `ROADMAP.md`.

// No Windows, um programa gráfico: sem a janela de console ao abrir.
#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(any(target_os = "linux", windows))]
mod app;
#[cfg(windows)]
mod windows;

#[cfg(any(target_os = "linux", windows))]
fn main() {
    match terraforme_core::pet::claim() {
        Ok(true) => {}
        // Já existe uma janela aberta.
        Ok(false) => return,
        Err(e) => fail(&format!("{e:#}")),
    }
    let result = app::run();
    terraforme_core::pet::release();
    if let Err(e) = result {
        fail(&e.to_string());
    }
}

/// Sai com erro. No Windows não há console para o `eprintln!`, então o erro
/// também vai numa notificação.
#[cfg(any(target_os = "linux", windows))]
fn fail(message: &str) -> ! {
    eprintln!("terraforme-pet: {message}");
    if cfg!(windows) {
        terraforme_core::pet::notify(&format!("o mascote não abriu: {message}"));
    }
    std::process::exit(1);
}

#[cfg(not(any(target_os = "linux", windows)))]
fn main() {
    eprintln!(
        "terraforme-pet: a janela flutuante ainda não existe neste sistema — veja o ROADMAP.md"
    );
    std::process::exit(1);
}
