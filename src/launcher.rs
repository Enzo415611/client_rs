use lighty_launcher::launch::InstanceControl;
use lighty_launcher::{Authenticator, Launch, Loader, event::EventBus, version};

use parking_lot::Mutex;
use slint::{
    ComponentHandle, Model, ModelExt, ModelRc, SharedString, ToSharedString, VecModel, Weak,
};
use std::{env::consts::OS, path::PathBuf, sync::Arc};

use crate::slint_generatedAppWindow::LoaderS;
use crate::{AppState, AppWindow, Instance, Logic, NewInstance};

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

async fn create_instance(
    weak: Weak<AppWindow>,
    app_state: Arc<Mutex<AppState>>,
    new_instance: NewInstance,
) -> anyhow::Result<()> {
    println!("init instance");
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    lighty_launcher::_core::AppState::init(".minecraft")?;

    let instance = version::VersionBuilder::new(
        &new_instance.name,
        new_instance.loader.clone(),
        &new_instance.loader_version,
        &new_instance.version,
    );

    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();

        let mut instances_vec = vec![];

        _ = logic.get_instances().iter().map(|i| instances_vec.push(i));

        instances_vec.push(crate::InstanceS {
            name: SharedString::from(new_instance.name),
            version: SharedString::from(new_instance.version),
            loader: to_slint_loader_enum(&new_instance.loader),
            loader_version: SharedString::from(new_instance.loader_version),
            is_run: false,
        });

        logic.set_instances(ModelRc::new(VecModel::from(instances_vec)));
    }

    app_state.lock().instances.push(Instance {
        version_builder: instance,
        is_run: false,
    });

    slint::spawn_local(async_compat::Compat::new(async move {
        let app_state = app_state.clone();
        while let Ok(event) = rx.next().await {
            match event {
                lighty_launcher::event::Event::Launch(
                    lighty_launcher::event::LaunchEvent::InstallProgress { bytes },
                ) => {
                    // progresso ao baixar os arquivos do jogo
                    println!("   +{}", bytes);
                }
                lighty_launcher::event::Event::ConsoleOutput(out) => {
                    // log do jogo ao inicar e ao sair

                    println!("{}", out.line)
                }
                lighty_launcher::event::Event::Launch(
                    lighty_launcher::event::LaunchEvent::Launched { version, pid },
                ) => {}
                _ => {}
            }
        }
    }))
    .ok();

    // instance
    //     .launch(&profile, lighty_launcher::JavaDistribution::Temurin)
    //     .with_event_bus(&event_bus)
    //     .run()
    //     .await?;

    Ok(())
}

pub fn new_instance_thread(
    weak: Weak<AppWindow>,
    app_state: Arc<Mutex<AppState>>,
    new_instance: NewInstance,
) {
    slint::spawn_local(async_compat::Compat::new(create_instance(
        weak,
        app_state,
        new_instance,
    )))
    .ok();
}

fn get_instance_by_name(app_state: Arc<Mutex<AppState>>, name: &str) -> Option<Instance> {
    app_state.lock().instances.iter().find(|i| i.version_builder.name == name.to_string()).cloned()
}

fn on_run_instance(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();

        logic.on_run_instance(move |name| {
            if let Some(instance) = get_instance_by_name(app_state.clone(), &name) {
                
            }
        });
    }
}