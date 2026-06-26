#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher;

use std::{error::Error, sync::{Arc}};



use lighty_launcher::{Loader, VersionBuilder};
use parking_lot::Mutex;
use slint::{ModelRc, SharedString, VecModel, Weak};

use crate::launcher::{get_minecraft_dir, new_instance_thread};

slint::include_modules!();

struct AppState {
    instances: Vec<VersionBuilder<Loader>>,
}

struct Instance {
    pid: u32,
    name: String,
    version: String,
    loader_version: String,
    loader: Loader,
    is_run: bool
}

impl AppState {
    fn new() -> Self {
        Self {
            instances: vec![],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();
        
    let app_state = Arc::new(Mutex::new(AppState::new()));
    
    let weak = ui.as_weak();
    on_run_instance(weak, logic, app_state);
    // new_instance_thread(weak, app_state.clone());

    ui.run()?;
    Ok(())
}


fn on_run_instance(weak: Weak<AppWindow>, logic: Logic, app_state: Arc<Mutex<AppState>>) {
    logic.on_run_instance(move|instance| {
        let new_instance = Instance {
            pid: 0,
            name: instance.name.to_string(),
            version: instance.version.to_string(),
            loader: Loader::Vanilla,
            loader_version: instance.loader_version.to_string(),
            is_run: true
        };
        
        new_instance_thread(weak.clone(), app_state.clone(), new_instance);
    });
}