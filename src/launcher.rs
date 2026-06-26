use lighty_launcher::{
    Authenticator, Launch, Loader,
    core::{AppState},
    version,
};
use std::{env::consts::OS, path::PathBuf};

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

pub async fn init_launcher() -> anyhow::Result<()> {
    println!("init minecraft");
    
    AppState::init(".minecraft")?;
    let mut auth = lighty_launcher::auth::OfflineAuth::new("zzz");
    let profile = auth.authenticate().await?;
    let mut version = version::VersionBuilder::new("my-instance", Loader::Vanilla, "", "1.21.11");

    version
        .launch(&profile, lighty_launcher::JavaDistribution::Temurin)
        .run()
        .await?;

    Ok(())
}
