use windows::UI::Notifications::{
  NotificationData, NotificationUpdateResult, ToastNotificationManager, ToastNotifier,
};

use crate::NotifError;

use super::NotificationDataSet;

pub struct ToastsNotifier {
  _inner: ToastNotifier,
}

impl ToastsNotifier {
  pub fn new(app_id: &str) -> Result<Self, NotifError> {
    let _inner = ToastNotificationManager::CreateToastNotifierWithId(&app_id.into())?;

    Ok(Self { _inner })
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
