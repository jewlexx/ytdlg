pub const CHECK_SUM: &str = {
    cfg_if::cfg_if! {
        if #[cfg(not(target_arch = "x86_64"))] {
            compile_error!("Currently only x86_64 is supported");
        } else if #[cfg(windows)] {
            include_str!("youtube-dl.exe.sum")
        } else {
            include_str!("youtube-dl.sum")
        }
    }
};
