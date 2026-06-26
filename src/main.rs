#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher;

use std::error::Error;

use tokio::task::{spawn_blocking, spawn_local};

use crate::launcher::{get_minecraft_dir, init_launcher};

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let dir = get_minecraft_dir();
    println!("{}", dir.display().to_string());

    // deve estar dentro de uma thread async
    // a ui não ira abrir a não ser que o jogo seja fechado
    _ = init_launcher().await?;

    ui.run()?;
    Ok(())
}
