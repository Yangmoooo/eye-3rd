pub enum NotifyType {
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
                NotifyType::Info => format!("😼 eye³ v{version}"),
                NotifyType::Ok => format!("😻 eye³ v{version}"),
                NotifyType::Err => format!("😿 eye³ v{version}"),
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
