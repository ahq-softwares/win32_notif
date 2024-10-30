use std::collections::HashMap;

use crate::NotifError;

use super::{ToXML, ToastsNotifier};
use actions::ActionElement;
use audio::Audio;
use header::Header;
use visual::VisualElement;
use widgets::commands::Commands;
use windows::{
  core::HSTRING,
  Data::Xml::Dom::XmlDocument,
  UI::Notifications::{NotificationData, ToastNotification},
};

mod widgets;
pub use widgets::*;

/// The Notification Object
pub struct Notification<'a> {
  _toast: ToastNotification,
  _notifier: &'a ToastsNotifier,
}

impl<'a> Notification<'a> {
  pub fn show(&self) -> Result<(), NotifError> {
    Ok(self._notifier.get_raw_handle().Show(&self._toast)?)
  }

  #[cfg(feature = "unsafe")]
  /// Required Features: unsafe
  pub unsafe fn as_raw(&self) -> &ToastNotification {
    &self._toast
  }
}

pub trait ActionableXML: ActionElement + ToXML {}
pub trait ToastVisualableXML: VisualElement + ToXML {}

/// The way to build a Notification
pub struct NotificationBuilder {
  audio: Option<Audio>,
  header: Option<Header>,
  commands: Option<Commands>,
  visual: Vec<Box<dyn ToastVisualableXML>>,
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

  pub fn value<T: Into<String>, E: Into<String>>(mut self, key: T, value: E) -> Self {
    self.values.insert(key.into(), value.into());
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

  pub fn visual<T: ToastVisualableXML + 'static>(mut self, visual: T) -> Self {
    self.visual.push(Box::new(visual));
    self
  }

  pub fn visuals(mut self, visual: Vec<Box<dyn ToastVisualableXML>>) -> Self {
    self.visual = visual;
    self
  }

  pub fn build<'a>(
    self,
    sequence: u32,
    _notifier: &'a ToastsNotifier,
    tag: &str,
    group: &str,
  ) -> Result<Notification<'a>, NotifError> {
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

    println!("{_xml}");

    let doc = XmlDocument::new()?;
    doc.LoadXml(&HSTRING::from(_xml))?;

    let data = NotificationData::new()?;
    data.SetSequenceNumber(sequence)?;

    for (key, value) in self.values {
      data.Values()?.Insert(&key.into(), &value.into())?;
    }

    let toast = ToastNotification::CreateToastNotification(&doc)?;
    toast.SetTag(&tag.into())?;
    toast.SetGroup(&group.into())?;
    toast.SetData(&data)?;

    Ok(Notification {
      _toast: toast,
      _notifier,
    })
  }
}
