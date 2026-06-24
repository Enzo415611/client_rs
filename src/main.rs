#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher;

use std::error::Error;

use crate::launcher::launcher_thread;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    launcher_thread().await;
    
    if let Some(dir) = dirs::config_dir() {
        println!("{:?}", dir.join(".minecraft"));
    }
    
    ui.run()?;
    Ok(())
}
