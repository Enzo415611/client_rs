#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher;
mod auth;

use std::{error::Error, sync::Arc};

use lighty_launcher::{
    Loader, UserProfile, VersionBuilder,
    auth::{MicrosoftAuth, OfflineAuth},
    launch::InstanceControl,
};

use parking_lot::Mutex;
use slint::Weak;

use crate::{auth::{on_create_offline_account, on_create_online_account}, launcher::create_instance};

slint::include_modules!();

struct AppState {
    current_account: Option<UserProfile>,
    accounts: Vec<UserProfile>,
    accounts_for_slint: Vec<String>,
    current_accounts_mode: ProfileEnum,
    instances: Vec<Instance>,
    instances_for_slint: Vec<InstanceS>,
}

impl AppState {
    fn new() -> Self {
        Self {
            current_account: None,
            current_accounts_mode: ProfileEnum::Offline,
            accounts: vec![],
            accounts_for_slint: vec![],
            instances: vec![],
            instances_for_slint: vec![],
        }
    }
}

enum ProfileEnum {
    Online,
    Offline,
}

#[derive(Clone)]
struct Instance {
    version_builder: VersionBuilder<Loader>,
    is_run: bool,
}




fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    lighty_launcher::_core::AppState::init(".minecraft")?;
    let _logic = ui.global::<Logic>();
    let weak: Weak<AppWindow> = ui.as_weak();
    let app_state = Arc::new(Mutex::new(AppState::new()));
    
    slint_callbacks(weak, app_state);
    ui.run()?;
    Ok(())
}

fn slint_callbacks(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    on_create_instance(weak.clone(), app_state.clone());
    on_create_online_account(weak.clone(), app_state.clone());
    on_create_offline_account(weak.clone(), app_state.clone());
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

fn on_create_instance(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();
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
    
}
