use std::path::PathBuf;

use crate::consts::BIN_PATH;
use tokio::sync::mpsc::Receiver;

pub struct VideoDownloadInfo {
    pub url: String,
    pub file_path: PathBuf,
    pub format_id: String,
}

pub fn spawn_dl_thread<T>(mut rx: Receiver<T>)
where
    T: std::fmt::Display + Send + 'static,
{
    tokio::spawn(async move {
        while let Ok(msg) = rx.try_recv() {
            let msg_string = msg.to_string();
            println!("{}", msg);
        }
    });
}
