use std::collections::HashMap;

use super::ToXML;
use actions::ActionElement;
use audio::Audio;
use header::Header;
use widgets::commands::Commands;
use windows::{
  core::HSTRING,
  Data::Xml::Dom::XmlDocument,
  UI::Notifications::{NotificationData, ToastNotification},
};

mod widgets;
pub use widgets::*;

/// The Notification Object
pub struct Notification {
  _toast: ToastNotification,
}

impl Notification {
  pub fn show(&self) {}
}

pub trait ActionableXML: ActionElement + ToXML {}

/// The way to build a Notification
pub struct NotificationBuilder {
  audio: Option<Audio>,
  header: Option<Header>,
  commands: Option<Commands>,
  visual: Vec<Box<dyn ToXML>>,
  actions: Vec<Box<dyn ActionableXML>>,
  pub values: HashMap<String, String>,
}

macro_rules! impl_mut {
  ($x:ident -> $y:tt) => {
    pub fn $x(mut self, $x: $y) -> Self {
      self.$x = Some($x);
      self
    }
  };
}

#[macro_export]
macro_rules! map {
  ($x:expr) => {
    $x.into_iter()
      .map(|x| x.to_xml())
      .collect::<Vec<_>>()
      .join("\n".into())
  };
}

impl NotificationBuilder {
  pub fn new() -> Self {
    Self {
      visual: vec![],
      actions: vec![],
      audio: None,
      commands: None,
      header: None,
      values: HashMap::new(),
    }
  }

  impl_mut!(audio -> Audio);
  impl_mut!(header -> Header);
  impl_mut!(commands -> Commands);

  pub fn value(mut self, key: String, value: String) -> Self {
    self.values.insert(key, value);
    self
  }

  pub fn values(mut self, values: HashMap<String, String>) -> Self {
    self.values = values;
    self
  }

  pub fn action<T: ActionableXML + 'static>(mut self, action: T) -> Self {
    self.actions.push(Box::new(action));
    self
  }

  pub fn actions(mut self, actions: Vec<Box<dyn ActionableXML>>) -> Self {
    self.actions = actions;
    self
  }

  pub fn build(self, sequence: u32) -> Result<Notification, BuilderError> {
    let visual = map!(self.visual);
    let actions = map!(self.actions);

    let audio = self.audio.map_or_else(|| "".into(), |x| x.to_xml());
    let header = self.header.map_or_else(|| "".into(), |x| x.to_xml());

    let commands = self.commands.map_or_else(
      || "".into(),
      |x| {
        format!(
          r"
        <commands>
          {}
        </commands>
      ",
          map!(x)
        )
      },
    );

    let _xml = format!(
      r#"
      <toast>
        {audio}
        {commands}
        {header}
        <visual>
          <binding template='ToastGeneric'>
            {visual}
          </binding>
        </visual>
        <actions>
          {actions}
        </actions>
      </toast>
    "#
    );

    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from(_xml))?;

    let data = NotificationData::new()?;
    data.SetSequenceNumber(sequence)?;

    todo!()
  }
}

macro_rules! from_impl {
  ($x:ty => $y:ident) => {
    impl From<$x> for BuilderError {
      fn from(value: $x) -> Self {
        Self::$y(value)
      }
    }
  };
}

pub enum BuilderError {
  WindowsCore(windows::core::Error),
}

from_impl!(windows::core::Error => WindowsCore);
