pub enum MsgType {
    Info,
    Ok,
    Err,
}

#[macro_export]
macro_rules! notify {
    ($ty:expr, $($arg:tt)*) => {
        {
            use notify_rust::{Notification, Timeout};
            let version = env!("CARGO_PKG_VERSION");
            let summary = match $ty {
                MsgType::Info => format!("😼 eye³ v{version}"),
                MsgType::Ok => format!("😻 eye³ v{version}"),
                MsgType::Err => format!("😿 eye³ v{version}"),
            };
            let msg = format!($($arg)*);
            let _ = Notification::new()
                .summary(&summary)
                .body(&msg)
                .timeout(Timeout::Milliseconds(4000))
                .show();
        }
    };
}
