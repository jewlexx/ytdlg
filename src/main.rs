#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const BIN_NAME: &str = {
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

fn main() {
    println!("Hello, world!");
}
