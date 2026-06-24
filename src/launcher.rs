use std::{
    path::PathBuf,
};
use lighty_launcher::{Authenticator, Launch, Loader, core::AppState, version};
fn get_minecraft_dir() -> PathBuf {
    if let Some(dir) = dirs::config_dir() {
        dir.join(".minecraft")
    } else {
        PathBuf::new()
    }
}

async fn init_launcher() -> anyhow::Result<()> {
    println!("init");
    AppState::init("launcher")?;
    let minecraft_dir = get_minecraft_dir().display().to_string();

    let mut auth = lighty_launcher::auth::OfflineAuth::new("zzz");
    let profile = auth.authenticate().await?;
    let mut version = version::VersionBuilder::new("zzz", Loader::Vanilla, "", "1.21.11");
    version.launch(&profile, lighty_launcher::JavaDistribution::Temurin).run().await?;

    Ok(())
}

pub async fn launcher_thread() {
    _=tokio::spawn(async {
        _=init_launcher().await;
    }).await;
}
