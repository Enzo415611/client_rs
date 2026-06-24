use std::{
    path::PathBuf,
    process::{Stdio},
    thread,
};

use async_process::Command;
use futures_lite::{AsyncBufReadExt, StreamExt, io::BufReader};
use li
fn get_minecraft_dir() -> PathBuf {
    if let Some(dir) = dirs::config_dir() {
        dir.join(".minecraft")
    } else {
        PathBuf::new()
    }
}

async fn init_launcher() -> anyhow::Result<()> {
    println!("init");

    let minecraft_dir = get_minecraft_dir();

    let mut auth = 
    // let mut child = Command::new(&command.executable)
    //     .args(&command.args)
    //     .current_dir(&command.working_dir)
    //     .stdout(Stdio::piped())
    //     .spawn()?;

    // let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

    // while let Some(line) = lines.next().await {
    //     println!("{}", line?)
    // }
    Ok(())
}

pub async fn launcher_thread() {
    tokio::spawn(async {
        init_launcher()
    }).await;
}
