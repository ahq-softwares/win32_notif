//! Win32 Notification
//!
//! This library implements UWP XML Toast Notification
//! This is a safe wrapper around the official C++ win32 apis

#[macro_export]
///
/// Creates a reference to a value in notification
///
/// # Example
/// ```rust
/// use win32_notif::string;
///
/// fn main() {
///     let value = string!("status");
/// }
/// ```
macro_rules! string {
    ($($x:tt)*) => {
        format!($($x)*)
    };
}

#[macro_export]
///
/// Creates a reference to a value in notification
///
/// # Example
/// ```rust
/// use win32_notif::refvar;
///
/// fn main() {
///     let value = refvar!(status);
/// }
/// ```
macro_rules! refvar {
    ($($x:tt)*) => {
        format!("{{{}}}", stringify!($($x)*))
    };
}

mod structs;

pub use structs::*;
