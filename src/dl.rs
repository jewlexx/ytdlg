use std::path::PathBuf;

use crate::consts::BIN_PATH;
use tokio::{
    process::Command,
    sync::{mpsc::Receiver, watch::Sender},
};

#[derive(Debug)]
pub struct VideoDownloadInfo {
    pub url: String,
    pub file_path: Option<PathBuf>,
    pub format_id: String,
}

pub fn spawn_dl_thread(mut rx: Receiver<VideoDownloadInfo>, tx: Sender<bool>) {
    tokio::spawn(async move {
        loop {
            if let Ok(msg) = rx.try_recv() {
                let msg_string = msg;

                let output_args = if let Some(ref path) = msg_string.file_path {
                    vec!["-o", path.as_os_str().to_str().unwrap()]
                } else {
                    vec![]
                };

                println!("downloading");

                let out = Command::new(BIN_PATH.clone())
                    .args(&["-f", &msg_string.format_id, &msg_string.url])
                    .args(&output_args)
                    .output()
                    .await
                    .unwrap();

                println!("{}", String::from_utf8_lossy(&out.stdout));

                // Flips it so that the other thread knows we are finished
                tx.send(!*tx.borrow()).unwrap();
            }
        }
    });
}
