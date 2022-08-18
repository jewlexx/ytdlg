use std::path::PathBuf;

use crate::consts::BIN_PATH;
use tokio::sync::{mpsc::Receiver, watch::Sender};

pub struct VideoDownloadInfo {
    pub url: String,
    pub file_path: PathBuf,
    pub format_id: String,
}

pub fn spawn_dl_thread(mut rx: Receiver<VideoDownloadInfo>, tx: Sender<()>) {
    tokio::spawn(async move {
        while let Ok(msg) = rx.try_recv() {
            let msg_string = msg;
            tx.send(()).unwrap();
        }
    });
}
