pub const BIN_NAME: &str = {
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

pub const BIN_DOWNLOAD_URL: &str =
    const_format::concatcp!("https://youtube-dl.org/downloads/2021.12.17/", BIN_NAME);

lazy_static::lazy_static! {
    pub static ref BIN_PATH: std::path::PathBuf = dirs::cache_dir()
            .expect("failed to find cache dir")
            .join("ytdlg")
            .join(BIN_NAME);
}
