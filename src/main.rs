#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// TODO: Build in checksums

use std::{fs::File, io::Read};

const BIN_NAME: &str = {
    const EXT: &str = {
        cfg_if::cfg_if! {
            if #[cfg(not(target_arch = "x86_64"))] {
                compile_error!("Currently only x86_64 is supported");
            } else if #[cfg(windows)] {
                ".exe"
            } else {
                ""
            }
        }
    };

    const_format::concatcp!("youtube-dl", EXT)
};

const CHECK_SUM: &str = {
    cfg_if::cfg_if! {
        if #[cfg(not(target_arch = "x86_64"))] {
            compile_error!("Currently only x86_64 is supported");
        } else if #[cfg(windows)] {
            include_str!("sums/youtube-dl.exe.sum")
        } else {
            include_str!("sums/youtube-dl.sum")
        }
    }
};

const BIN_DOWNLOAD_URL: &str =
    const_format::concatcp!("https://youtube-dl.org/downloads/2021.12.17/", BIN_NAME);

lazy_static::lazy_static! {
    static ref BIN_PATH: std::path::PathBuf = dirs::cache_dir()
            .expect("failed to find cache dir")
            .join("ytdlg")
            .join(BIN_NAME);
}

fn check_integrity() -> Result<(), &'static str> {
    use sha2::{Digest, Sha256};

    let mut target_file = File::open(BIN_PATH.clone()).unwrap();
    let mut target_bytes = Vec::new();
    target_file.read_to_end(&mut target_bytes).unwrap();

    let mut hasher = Sha256::new();

    hasher.update(&target_bytes);

    let response = hasher.finalize();
    let mut sum_hex = [0; 32];

    hex::decode_to_slice(CHECK_SUM, &mut sum_hex).unwrap();

    if response[..] == sum_hex {
        Ok(())
    } else {
        Err("Checksum mismatch")
    }
}

fn dl_binary() {
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
    }

    check_integrity().expect("integrity check failed");
}

fn main() {
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
    text: String,
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.text_edit_singleline(&mut self.text));
    }
}
