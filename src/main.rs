#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// TODO: Build in checksums

use std::fs::File;

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

const BIN_DOWNLOAD_URL: &str =
    const_format::concatcp!("https://youtube-dl.org/downloads/2021.12.17/", BIN_NAME);

lazy_static::lazy_static! {
    static ref BIN_PATH: std::path::PathBuf = dirs::cache_dir()
            .expect("failed to find cache dir")
            .join("ytdlg")
            .join(BIN_NAME);
}

fn main() {
    let mut target_file = File::create(BIN_PATH.clone()).unwrap();

    reqwest::blocking::get(BIN_DOWNLOAD_URL)
        .unwrap()
        .copy_to(&mut target_file)
        .unwrap();

    println!("Hello, world!");
}
