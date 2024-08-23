#![cfg_attr(docsrs, feature(doc_auto_cfg))]

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

use std::{error::Error, fmt::Display};

pub use structs::*;

macro_rules! from_impl {
  ($x:ty => $y:ident) => {
    impl From<$x> for NotifError {
      fn from(value: $x) -> Self {
        Self::$y(value)
      }
    }
  };
}
  
#[derive(Debug)]
pub enum NotifError {
  WindowsCore(windows::core::Error),
}

impl Display for NotifError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for NotifError {}
  
from_impl!(windows::core::Error => WindowsCore);