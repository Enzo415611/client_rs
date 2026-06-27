#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher;

use std::{error::Error, sync::{Arc}};



use lighty_launcher::{Loader, VersionBuilder, auth::{MicrosoftAuth, OfflineAuth}, launch::InstanceControl};
use parking_lot::Mutex;
use slint::{Weak};

use crate::launcher::{new_instance_thread};

slint::include_modules!();

struct AppState {
    offline_auth: Option<OfflineAuth>,
    online_auth: Option<MicrosoftAuth>,
    instances: Vec<Instance>,
    simple_instances: Vec<NewInstance>
}

#[derive(Clone)]
struct Instance {
    version_builder: VersionBuilder<Loader>,
    is_run: bool,
}

struct NewInstance {
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
            offline_auth: None,
            online_auth: None,
            instances: vec![],
            simple_instances: vec![]
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let logic = ui.global::<Logic>();
        
    let app_state = Arc::new(Mutex::new(AppState::new()));
    
    let weak: Weak<AppWindow> = ui.as_weak();
    on_create_instance(weak, logic, app_state);

    ui.run()?;
    Ok(())
}


fn on_create_instance(weak: Weak<AppWindow>, logic: Logic, app_state: Arc<Mutex<AppState>>) {
    
    logic.on_create_instance(move|instance| {
        let new_instance = NewInstance {
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