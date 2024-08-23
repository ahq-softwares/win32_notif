pub mod notification;
pub mod notifier;

pub use notifier::ToastsNotifier;
pub use notification::{Notification, NotificationBuilder};

pub trait ToXML {
  fn to_xml(&self) -> String;
}
