#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs::File;

const BIN_NAME: &str = {
    const EXT: &str = {
        cfg_if::cfg_if! {
            if #[cfg(not(target_arch = "x86_64"))] {
                compile_error!("Currently only x86_64 is supported");
            } else if #[cfg(windows)] {
                "youtube-dl.exe"
            } else {
                "youtube-dl"
            }
        }
    };

    const_format::concatcp!("youtube-dl", EXT)
};

const BIN_DOWNLOAD_URL: &str =
    const_format::concatcp!("https://youtube-dl.org/downloads/latest/", BIN_NAME);

fn main() {
    let mut target_file = File::create(BIN_NAME).unwrap();

    reqwest::blocking::get(BIN_DOWNLOAD_URL)
        .unwrap().copy_to(&mut target_file)
        .unwrap()

    println!("Hello, world!");
}
