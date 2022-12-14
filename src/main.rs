#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use dl::{spawn_dl_thread, VideoDownloadInfo};
use native_dialog::MessageType;
use parking_lot::Mutex;
use poll_promise::Promise;
use quork::LockMap;
use tokio::process::Command;
use ytdl::{dl_binary, YtdlManifest};

mod consts;
mod dl;
mod sums;
mod ytdl;

fn panic_hook(info: &std::panic::PanicInfo) {
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
}

#[tokio::main]
async fn main() {
    std::panic::set_hook(Box::new(panic_hook));

    dl_binary();

    let (dl_spawn, dl_recv) = tokio::sync::mpsc::channel::<VideoDownloadInfo>(10);
    let (dl_fin_send, dl_finished) = tokio::sync::watch::channel::<bool>(false);

    spawn_dl_thread(dl_recv, dl_fin_send);

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Download and show an image with eframe/egui",
        options,
        Box::new(|_cc| {
            Box::new(Application {
                yt_url: String::new(),
                is_downloading: false,
                manifest: None,
                dl_thread: None,
                dl_sender: dl_spawn,
                dl_receiver: dl_finished,
            })
        }),
    );
}

struct Application {
    yt_url: String,
    is_downloading: bool,
    manifest: Option<Promise<YtdlManifest>>,
    dl_thread: Option<Promise<()>>,
    dl_sender: tokio::sync::mpsc::Sender<dl::VideoDownloadInfo>,
    dl_receiver: tokio::sync::watch::Receiver<bool>,
}

struct DownloadStatus(pub u64, pub u64);

impl DownloadStatus {
    pub fn set_progress(&mut self, progress: u64) {
        self.0 = progress;
    }
    pub fn set_total(&mut self, total: u64) {
        self.1 = total;
    }
}

static BIN_DOWNLOAD: Mutex<DownloadStatus> = Mutex::new(DownloadStatus(0, 1));

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (downloaded, total) = BIN_DOWNLOAD.lock().map(|d| (d.0, d.1));

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

                    let _ = self.manifest.insert(Promise::spawn_async(async {
                        let path = consts::BIN_PATH.clone();
                        let mut cmd = Command::new(path);
                        cmd.arg(url).arg("--dump-json");

                        let out = cmd.output().await.expect("failed to get output");

                        serde_json::from_slice(&out.stdout).expect("invalid response")
                    }));

                    self.is_downloading = true;
                    // println!("{}", self.yt_url);
                }

                if let Some(manifest) = self.manifest.as_ref().and_then(|p| p.ready().cloned()) {
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

                                if dl_btn.clicked() {
                                    let format = format.clone();
                                    let dl_url = self.yt_url.clone();
                                    let sender = self.dl_sender.clone();
                                    let mut recv = self.dl_receiver.clone();
                                    self.dl_thread.replace(Promise::spawn_async(async move {
                                        sender
                                            .send(VideoDownloadInfo {
                                                url: dl_url,
                                                file_path: None,
                                                format_id: format.format_id.clone(),
                                            })
                                            .await
                                            .err();

                                        recv.changed().await.unwrap();
                                        println!("downloaded");
                                    }));
                                }
                            });
                        }
                    }
                }
            });
        });
    }
}
