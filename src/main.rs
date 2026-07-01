#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod instance;

use std::{error::Error, sync::Arc};

use dotenvy::dotenv;
use lighty_launcher::{
    Loader, UserProfile, VersionBuilder,
    auth::{MicrosoftAuth, OfflineAuth},
    launch::InstanceControl,
};

use parking_lot::Mutex;
use slint::Weak;

use crate::{
    auth::{on_create_offline_account, on_login_offline_account, on_login_online_account},
    instance::{Instance, on_create_instance},
};

slint::include_modules!();

struct AppState {
    current_account: Option<UserProfile>,
    accounts: Vec<UserProfile>,
    accounts_for_slint: Vec<String>,
    instances: Vec<Instance>,
    instances_for_slint: Vec<InstanceS>,
}

impl AppState {
    fn new() -> Self {
        Self {
            current_account: None,
            accounts: vec![],
            accounts_for_slint: vec![],
            instances: vec![],
            instances_for_slint: vec![],
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
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
    on_login_online_account(weak.clone(), app_state.clone());
    on_create_offline_account(weak.clone(), app_state.clone());
    on_login_offline_account(weak.clone(), app_state.clone());
}
