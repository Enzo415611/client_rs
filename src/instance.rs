use std::{env::consts::OS, path::PathBuf, sync::Arc};

use lighty_launcher::{
    Launch, Loader, VersionBuilder, event::EventReceiver, loaders::VersionInfo, version,
};
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

            _ = create_instance(weak.clone(), app_state.clone(), new_instance);
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

fn get_instance_by_name(app_state: Arc<Mutex<AppState>>, name: &str) -> Option<Instance> {
    app_state
        .lock()
        .instances
        .iter()
        .find(|i| i.version_builder.name == name.to_string())
        .cloned()
}

pub fn on_run_instance(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();
        logic.on_run_instance(move |name| {
            println!("run");

            if let Some(mut instance) = get_instance_by_name(app_state.clone(), &name) {
                println!("{:?}", instance.version_builder.game_dirs);

                let app_state = app_state.clone();
                slint::spawn_local(async_compat::Compat::new(async move {
                    if let Some(user) = &app_state.lock().current_account {
                        _ = instance
                            .version_builder
                            .launch(user, lighty_launcher::JavaDistribution::Temurin)
                            .run()
                            .await;
                    }

                    // match app_state.lock().rx.next().await.unwrap() {
                    //     lighty_launcher::event::Event::Launch(event) => match event {
                    //         lighty_launcher::event::LaunchEvent::ProcessOutput {
                    //             pid,
                    //             stream,
                    //             line,
                    //         } => {
                    //             println!("ProcessOutput: stram: {}, line: {}", stream, line);
                    //         }
                    //         lighty_launcher::event::LaunchEvent::InstallCompleted {
                    //             version,
                    //             total_bytes,
                    //         } => {
                    //             println!("InstallCompleted: {}", total_bytes)
                    //         }
                    //         lighty_launcher::event::LaunchEvent::InstallProgress { bytes } => {
                    //             println!("{}", bytes);
                    //         }
                    //         lighty_launcher::event::LaunchEvent::Launched { version, pid } => {
                    //             println!("Launched: {}", pid)
                    //         }
                    //         lighty_launcher::event::LaunchEvent::NotLaunched { version, error } => {
                    //             println!("NotLaunched: {}", error);
                    //         }
                    //         _ => {}
                    //     },
                    //     _ => {}
                    // }
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
