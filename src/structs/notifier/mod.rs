use std::sync::Arc;

use windows::UI::Notifications::{
  NotificationData, NotificationUpdateResult, ToastNotificationHistory, ToastNotificationManager, ToastNotifier
};

use crate::NotifError;

use super::NotificationDataSet;

pub struct ToastsNotifier {
  _inner: ToastNotifier,
  app_id: Arc<String>
}

impl ToastsNotifier {
  pub fn new<T: Into<String>>(app_id: T) -> Result<Self, NotifError> {
    let string: String = app_id.into();
    let _inner = ToastNotificationManager::CreateToastNotifierWithId(&string.clone().into())?;

    Ok(Self { _inner, app_id: Arc::new(string) })
  }

  pub fn manager(&self) -> Result<ToastsManager, NotifError> {
    Ok(ToastsManager {
      inner: Arc::new(ToastNotificationManager::History()?),
      app_id: self.app_id.clone(),
    })
  }

  pub fn update(
    &self,
    data: &NotificationDataSet,
    group: &str,
    tag: &str,
  ) -> Result<NotificationUpdateResult, NotifError> {
    let raw: &NotificationData = data.inner_win32_type();
    Ok(
      self
        ._inner
        .UpdateWithTagAndGroup(raw, &tag.into(), &group.into())?,
    )
  }

  pub fn get_raw_handle(&self) -> &ToastNotifier {
    &self._inner
  }
}

pub type SafeString = String;

#[derive(Debug, Clone)]
pub struct ToastsManager {
  pub(crate) inner: Arc<ToastNotificationHistory>,
  pub app_id: Arc<SafeString>
}

impl ToastsManager {
  pub fn inner(&self) -> &Arc<ToastNotificationHistory> {
    &self.inner
  }
}