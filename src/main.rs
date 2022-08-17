#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs::File, io::Read, os::unix::prelude::PermissionsExt, process::Command};

use native_dialog::MessageType;
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

fn dl_binary() {
    use consts::{BIN_DOWNLOAD_URL, BIN_PATH};

    if !BIN_PATH.clone().exists() {
        {
            let mut dir_path = BIN_PATH.clone();
            dir_path.pop();
            std::fs::create_dir_all(dir_path).unwrap();
        }

        let mut target_file = File::create(BIN_PATH.clone()).unwrap();

        reqwest::blocking::get(BIN_DOWNLOAD_URL)
            .unwrap()
            .copy_to(&mut target_file)
            .unwrap();

        target_file
            .set_permissions(std::fs::Permissions::from_mode(0o755))
            .unwrap();
    }

    check_integrity().expect("integrity check failed");
}

fn main() {
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

    dl_binary();

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

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.text_edit_singleline(&mut self.yt_url);

                if self.is_downloading {
                    ui.spinner();
                }

                ui.add_space(5.);
                let btn = ui.button("Download options");

                if btn.clicked() {
                    let url = self.yt_url.clone();

                    let _ = self.manifest.insert(Promise::spawn_thread(
                        "download_manifest",
                        move || {
                            let path = consts::BIN_PATH.clone();
                            let mut cmd = Command::new(path);
                            cmd.arg(url).arg("--dump-json");

                            let out = cmd.output().expect("failed to get output");

                            serde_json::from_slice(&out.stdout).expect("invalid response")
                        },
                    ));

                    self.is_downloading = true;
                    println!("{}", self.yt_url);
                }

                if let Some(manifest) = self.manifest.as_ref().and_then(|p| p.ready()) {
                    self.is_downloading = false;
                    println!("{:?}", manifest);
                }
            });
        });
    }
}
