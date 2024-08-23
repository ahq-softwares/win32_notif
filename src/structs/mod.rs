pub mod notification;

pub use notification::{Notification, NotificationBuilder};

pub trait ToXML {
  fn to_xml(&self) -> String;
}
