use std::path::PathBuf;

use crate::consts::BIN_PATH;
use tokio::{
    process::Command,
    sync::{mpsc::Receiver, watch::Sender},
};

pub struct VideoDownloadInfo {
    pub url: String,
    pub file_path: PathBuf,
    pub format_id: String,
}

pub fn spawn_dl_thread(mut rx: Receiver<VideoDownloadInfo>, tx: Sender<()>) {
    tokio::spawn(async move {
        while let Ok(msg) = rx.try_recv() {
            let msg_string = msg;

            Command::new(BIN_PATH.clone())
                .args(&["-f", &msg_string.format_id, &msg_string.url])
                .arg(&msg_string.file_path)
                .output()
                .await
                .unwrap();

            tx.send(()).unwrap();
        }
    });
}
