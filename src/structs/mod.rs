pub mod data;
pub mod notification;
pub mod notifier;

pub use data::NotificationDataSet;
pub use notification::{Notification, NotificationBuilder};
pub use notifier::ToastsNotifier;

pub trait ToXML {
  fn to_xml(&self) -> String;
}
