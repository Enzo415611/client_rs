use std::{env::consts::OS, path::PathBuf, sync::Arc};

use lighty_launcher::{Launch, Loader, VersionBuilder, version};
use parking_lot::Mutex;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::{AppState, AppWindow, InstanceS, LoaderS, Logic};

#[derive(Clone)]
pub struct Instance {
    version_builder: VersionBuilder<Loader>,
    is_run: bool,
}

pub struct SimpleInstance {
    pub pid: u32,
    pub name: String,
    pub version: String,
    pub loader: Loader,
    pub loader_version: String,
    pub is_run: bool,
}

pub fn on_create_instance(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
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

            _=create_instance(weak.clone(), app_state.clone(), new_instance);
        });
    }
}


pub fn create_instance(
    weak: Weak<AppWindow>,
    app_state: Arc<Mutex<AppState>>,
    new_instance: SimpleInstance,
) -> anyhow::Result<()> {
    // let event_bus = EventBus::new(1000);
    // let mut rx = event_bus.subscribe();

    let instance = version::VersionBuilder::new(
        &new_instance.name,
        new_instance.loader.clone(),
        &new_instance.loader_version,
        &new_instance.version,
    );

    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();

        app_state.lock().instances_for_slint.push(InstanceS {
            name: SharedString::from(new_instance.name),
            version: SharedString::from(new_instance.version),
            loader: to_slint_loader_enum(&new_instance.loader),
            loader_version: SharedString::from(new_instance.loader_version),
            is_run: false,
        });
        logic.set_instances(ModelRc::new(VecModel::from(
            app_state.lock().instances_for_slint.clone(),
        )));
    }

    app_state.lock().instances.push(Instance {
        version_builder: instance,
        is_run: false,
    });
    Ok(())
}

pub fn get_minecraft_dir() -> PathBuf {
    match OS {
        "windows" => {
            if let Some(dir) = dirs::config_dir() {
                return dir.join(".minecraft");
            }
            PathBuf::new()
        }
        "macos" => PathBuf::new(),
        "linux" => {
            if let Some(dir) = dirs::data_dir() {
                return dir.join(".minecraft");
            }
            PathBuf::new()
        }
        _ => PathBuf::new(),
    }
}


fn get_instance_by_name(app_state: Arc<Mutex<AppState>>, name: &str) -> Option<Instance> {
    app_state
        .lock()
        .instances
        .iter()
        .find(|i| i.version_builder.name == name.to_string())
        .cloned()
}

fn on_run_instance(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();

        logic.on_run_instance(move |name| {
            if let Some(mut instance) = get_instance_by_name(app_state.clone(), &name) {
                let app_state = app_state.clone();
                slint::spawn_local(async_compat::Compat::new(async move {
                    if let Some(user) = &app_state.lock().current_account {
                        _ = instance
                            .version_builder
                            .launch(user, lighty_launcher::JavaDistribution::Temurin)
                            .run()
                            .await;
                    }
                }))
                .ok();
            }
        });
    }
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

fn to_slint_loader_enum(loader: &Loader) -> LoaderS {
    match loader {
        Loader::Vanilla => LoaderS::Vanilla,
        Loader::Fabric => LoaderS::Fabric,
        Loader::Forge => LoaderS::Forge,
        Loader::Optifine => LoaderS::Optifine,
        Loader::NeoForge => LoaderS::NeoForge,
        Loader::Quilt => LoaderS::Quilt,
        Loader::LightyUpdater => LoaderS::Vanilla,
    }
}

fn open_url(url: &str) {
    if let Err(err) = open::that(url) {
        eprintln!("{}", err);
    }
}
