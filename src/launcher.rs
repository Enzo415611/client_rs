use lighty_launcher::launch::InstanceControl;
use lighty_launcher::{Authenticator, Launch, Loader, event::EventBus, version};

use parking_lot::Mutex;
use slint::{ComponentHandle, Weak};
use std::{env::consts::OS, path::PathBuf, sync::Arc};

use crate::{AppState, AppWindow, Instance, Logic};

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

async fn run_new_instance(
    weak: Weak<AppWindow>,
    app_state: Arc<Mutex<AppState>>,
    new_instance: Instance,
) -> anyhow::Result<()> {
    println!("init instance");
    let event_bus = EventBus::new(1000);
    let mut rx = event_bus.subscribe();
    lighty_launcher::_core::AppState::init(".minecraft")?;
    let mut auth = lighty_launcher::auth::OfflineAuth::new("zzz");
    let profile = auth.authenticate(Some(&event_bus)).await?;

    let mut instance = version::VersionBuilder::new(
        &new_instance.name,
        new_instance.loader.clone(),
        &new_instance.loader_version,
        &new_instance.version,
    );

    app_state.lock().instances.push(Instance {
        pid: instance.get_pid().unwrap_or_else(|| 0),
        loader: new_instance.loader,
        loader_version: new_instance.loader_version,
        name: new_instance.name,
        version: new_instance.version,
        is_run: false
    });


    app_state.lock().instances_launched.push(instance.clone());

    
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
                ) => {                    
                }
                _ => {}
            }
        }
    }))
    .ok();

    let instance_builder = instance
        .launch(&profile, lighty_launcher::JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;

    Ok(())
}

pub fn new_instance_thread(
    weak: Weak<AppWindow>,
    app_state: Arc<Mutex<AppState>>,
    new_instance: Instance,
) {
    slint::spawn_local(async_compat::Compat::new(run_new_instance(
        weak,
        app_state,
        new_instance,
    )))
    .ok();
}
