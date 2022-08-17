#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs::File, io::Read, process::Command};

use indicatif::ProgressBar;
use native_dialog::MessageType;
use parking_lot::Mutex;
use poll_promise::Promise;
use ytdl::YtdlManifest;

mod consts;
mod sums;
mod ytdl;

fn check_integrity() -> Result<(), &'static str> {
    use sha2::{Digest, Sha256};

    use consts::BIN_PATH;

    let mut target_file = File::open(BIN_PATH.clone()).unwrap();
    let mut target_bytes = Vec::new();
    target_file.read_to_end(&mut target_bytes).unwrap();

    let mut hasher = Sha256::new();

    hasher.update(&target_bytes);

    let response = hasher.finalize();
    let mut sum_hex = [0; 32];

    hex::decode_to_slice(sums::CHECK_SUM, &mut sum_hex).unwrap();

    if response[..] == sum_hex {
        Ok(())
    } else {
        Err("Checksum mismatch")
    }
}

#[tokio::main]
async fn main() {
    std::panic::set_hook(Box::new(|info| {
        use std::fmt::Write;

        use native_dialog::MessageDialog;

        let mut msg = String::from("An error occurred:\n");

        if cfg!(debug_assertions) {
            writeln!(&mut msg, "   Panicked at: {}", info.location().unwrap()).unwrap();
        }

        if let Some(payload) = info.payload().downcast_ref::<&'static str>() {
            writeln!(&mut msg, "   With Message: '{}', ", payload).unwrap();
        }

        MessageDialog::default()
            .set_title("YTDLG Error")
            .set_type(MessageType::Error)
            .set_text(&msg)
            .show_alert()
            .unwrap_or_else(|_| {
                eprintln!("{}", info);
            });
    }));

    tokio::spawn(async {
        use consts::{BIN_DOWNLOAD_URL, BIN_PATH};
        use futures_util::stream::StreamExt;
        use std::io::Write;

        if !BIN_PATH.clone().exists() {
            {
                let mut dir_path = BIN_PATH.clone();
                dir_path.pop();
                std::fs::create_dir_all(dir_path).unwrap();
            }

            let mut target_file = File::create(BIN_PATH.clone()).unwrap();

            let res = reqwest::get(BIN_DOWNLOAD_URL).await.unwrap();
            let size = res.content_length().expect("failed to get content length");
            BIN_DOWNLOAD.lock().1 = size;
            let mut downloaded = 0;
            let mut stream = res.bytes_stream();

            while let Some(item) = stream.next().await {
                let chunk = item.unwrap();
                target_file.write_all(&chunk).unwrap();
                let new = std::cmp::min(downloaded + (chunk.len() as u64), size);
                downloaded = new;
                BIN_DOWNLOAD.lock().0 = new;
            }

            #[cfg(not(windows))]
            {
                use std::os::unix::prelude::PermissionsExt;
                target_file
                    .set_permissions(std::fs::Permissions::from_mode(0o755))
                    .unwrap();
            }
        }

        check_integrity().expect("integrity check failed");
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Download and show an image with eframe/egui",
        options,
        Box::new(|_cc| Box::new(Application::default())),
    );
}

#[derive(Default)]
struct Application {
    yt_url: String,
    is_downloading: bool,
    manifest: Option<Promise<YtdlManifest>>,
}

static BIN_DOWNLOAD: Mutex<(u64, u64)> = Mutex::new((0, 1));

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let downloaded = BIN_DOWNLOAD.lock().0;
            let total = BIN_DOWNLOAD.lock().1;
            if downloaded != total {
                let w = egui::ProgressBar::new((downloaded / total) as f32)
                    .animate(true)
                    .show_percentage();
                ui.add(w);
                return;
            }

            ui.vertical_centered(|ui| {
                ui.text_edit_singleline(&mut self.yt_url);

                if self.is_downloading {
                    ui.spinner();
                }

                ui.add_space(5.);
                let btn = ui.button("Download options");

                if btn.clicked() {
                    if let Some(p) = self.manifest.as_ref().and_then(|p| p.ready()) {
                        if p.webpage_url == Some(self.yt_url.clone()) {
                            return;
                        }
                    }

                    let url = self.yt_url.clone();

                    let _ = self.manifest.insert(Promise::spawn_blocking(|| {
                        let path = consts::BIN_PATH.clone();
                        let mut cmd = Command::new(path);
                        cmd.arg(url).arg("--dump-json");

                        let out = cmd.output().expect("failed to get output");

                        serde_json::from_slice(&out.stdout).expect("invalid response")
                    }));

                    self.is_downloading = true;
                    println!("{}", self.yt_url);
                }

                if let Some(manifest) = self.manifest.as_ref().and_then(|p| p.ready()) {
                    self.is_downloading = false;
                    if let Some(title) = &manifest.title {
                        ui.heading(title);
                    }

                    for format in &manifest.formats {
                        if format.width.is_some() {
                            ui.horizontal(|ui| {
                                let dl_btn = ui.button("Download this format");
                                ui.strong(format!("Fps: {}", format.fps.as_ref().unwrap()));

                                ui.strong(format!(
                                    "Resolution: {}x{}",
                                    format.width.as_ref().unwrap(),
                                    format.height.as_ref().unwrap()
                                ));
                            });
                        }
                    }
                }
            });
        });
    }
}
