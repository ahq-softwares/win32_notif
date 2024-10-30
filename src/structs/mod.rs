pub mod notification;
pub mod notifier;
pub mod data;

pub use data::NotificationDataSet;
pub use notifier::ToastsNotifier;
pub use notification::{Notification, NotificationBuilder};

pub trait ToXML {
  fn to_xml(&self) -> String;
}
