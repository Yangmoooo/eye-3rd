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
            let summary = match $ty {
                NotifyType::Info => "😼 eye³",
                NotifyType::Ok => "😻 eye³",
                NotifyType::Err => "😿 eye³",
            };
            let msg = format!($($arg)*);
            let _ = Notification::new()
                .summary(summary)
                .body(&msg)
                .timeout(Timeout::Milliseconds(4000))
                .show();
        }
    };
}
