#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher;

use std::{error::Error, sync::Arc};

use lighty_launcher::{
    Loader, UserProfile, VersionBuilder,
    auth::{MicrosoftAuth, OfflineAuth},
    launch::InstanceControl,
};
use parking_lot::Mutex;
use slint::Weak;

use crate::launcher::create_instance;

slint::include_modules!();

struct AppState {
    current_profile: Option<UserProfile>,
    current_profil_type: ProfileEnum,
    instances: Vec<Instance>,
    instances_for_slint: Vec<InstanceS>,
}

enum ProfileEnum {
    Online,
    Offline,
}

struct Profile {
    offline_auth: Option<OfflineAuth>,
    online_auth: Option<MicrosoftAuth>,
}

#[derive(Clone)]
struct Instance {
    version_builder: VersionBuilder<Loader>,
    is_run: bool,
}


impl AppState {
    fn new() -> Self {
        Self {
            current_profile: None,
            current_profil_type: ProfileEnum::Offline,
            instances: vec![],
            instances_for_slint: vec![],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    lighty_launcher::_core::AppState::init(".minecraft")?;
    let logic = ui.global::<Logic>();
    let app_state = Arc::new(Mutex::new(AppState::new()));
    let weak: Weak<AppWindow> = ui.as_weak();
    on_create_instance(weak, logic, app_state);

    ui.run()?;
    Ok(())
}

fn to_loader(loader: &LoaderS) -> Loader {
    match loader {
        LoaderS::Vanilla => Loader::Vanilla,
        LoaderS::Fabric => Loader::Fabric,
        LoaderS::Forge => Loader::Forge,
        LoaderS::Optifine => Loader::Optifine,
        LoaderS::NeoForge => Loader::NeoForge,
        LoaderS::Quilt => Loader::Quilt,
    }
}

struct SimpleInstance {
    pid: u32,
    name: String,
    version: String,
    loader: Loader,
    loader_version: String,
    is_run: bool
}

fn on_create_instance(weak: Weak<AppWindow>, logic: Logic, app_state: Arc<Mutex<AppState>>) {
    logic.on_create_instance(move |instance| {
        let new_instance: SimpleInstance = SimpleInstance {
            pid: 0,
            name: instance.name.to_string(),
            version: instance.version.to_string(),
            loader: to_loader(&instance.loader),
            loader_version: instance.loader_version.to_string(),
            is_run: false,
        };

        create_instance(weak.clone(), app_state.clone(), new_instance).ok();
    });
}
