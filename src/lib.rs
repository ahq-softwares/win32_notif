#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(private_bounds)]

//! Win32 Notification
//!
//! This library implements UWP XML Toast Notification
//! This is a safe wrapper around the official WinRT apis
//!
//! # Example
//! ```rust
//! use win32_notif::{
//!  notification::visual::progress::{Progress, ProgressValue},
//!  string, NotificationBuilder, ToastsNotifier,
//! };
//!
//! fn main() {
//!   let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();
//!   let notif = NotificationBuilder::new()
//!     .visual(Progress::new(
//!       None,
//!       string!("Downloading..."),
//!       ProgressValue::BindTo("prog"),
//!       None,
//!     ))
//!     // Use the newest data binding method
//!     .value("prog", "0.3")
//!     .build(1, &notifier, "a", "ahq")
//!     .unwrap();
//!
//!   let _ = notif.show();
//!   loop {}
//! }
//! ```

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

mod structs;

use std::{error::Error, fmt::Display};

pub use structs::*;

/// Re-export of windows crate
pub use windows;

macro_rules! from_impl {
  ($x:ty => $y:ident) => {
    impl From<$x> for NotifError {
      fn from(value: $x) -> Self {
        Self::$y(value)
      }
    }
  };
}

pub(crate) mod macros {
  #[macro_export]
  macro_rules! mktrait {
    (
      trait $name:ident {
        $($x:tt)*
      }
    ) => {
      #[cfg(feature = "unsafe")]
      pub trait $name {
        $($x)*
      }

      #[cfg(not(feature = "unsafe"))]
      pub(crate) trait $name {
        $($x)*
      }
    };
    (
      trait $name:ident: $($trait:tt),+ {
        $($x:tt)*
      }
    ) => {
      #[cfg(feature = "unsafe")]
      pub trait $name: $($trait)++ {
        $($x)*
      }

      #[cfg(not(feature = "unsafe"))]
      pub(crate) trait $name: $($trait)++ {
        $($x)*
      }
    };
  }
}

#[derive(Debug)]
pub enum NotifError {
  WindowsCore(windows::core::Error),
  DurationTooLong,
  UnknownAndImpossible,
}

impl Display for NotifError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for NotifError {}

from_impl!(windows::core::Error => WindowsCore);
