use tokio::sync::mpsc::Receiver;

pub fn spawn_dl_thread<T>(mut rx: Receiver<T>)
where
    T: std::fmt::Display + Send + 'static,
{
    tokio::spawn(async move {
        while let Ok(msg) = rx.try_recv() {
            let msg_string = msg.to_string();
            println!("{}", msg);
        }
    });
}
