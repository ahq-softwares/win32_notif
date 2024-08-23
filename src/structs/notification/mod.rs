use std::collections::HashMap;

use crate::NotifError;

use super::{ToXML, ToastsNotifier};
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
pub struct Notification<'a> {
  _toast: ToastNotification,
  _notifier: &'a ToastsNotifier
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

/// The way to build a Notification
pub struct NotificationBuilder {
  audio: Option<Audio>,
  header: Option<Header>,
  commands: Option<Commands>,
  visual: Vec<Box<dyn ToXML>>,
  actions: Vec<Box<dyn ActionableXML>>,
  pub values: HashMap<String, String>,
  pub app_id: String,
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
      app_id: "".into()
    }
  }

  impl_mut!(audio -> Audio);
  impl_mut!(header -> Header);
  impl_mut!(commands -> Commands);

  pub fn set_appid(mut self, appid: String) -> Self {
    self.app_id = appid;
    self
  }

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

  pub fn build<'a>(self, sequence: u32, _notifier: &'a ToastsNotifier, tag: &str, group: &str) -> Result<Notification<'a>, NotifError> {
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

    let toast = ToastNotification::CreateToastNotification(&doc)?;
    toast.SetData(&data)?;
    toast.SetTag(&tag.into())?;
    toast.SetGroup(&group.into())?;

    Ok(Notification { _toast: toast, _notifier })
  }
}
