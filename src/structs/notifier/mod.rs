use windows::UI::Notifications::{ToastNotificationManager, ToastNotifier};

use crate::NotifError;

pub struct ToastsNotifier {
  _inner: ToastNotifier
}

impl ToastsNotifier {
  pub fn new(app_id: &str) -> Result<Self, NotifError> {
    let _inner = ToastNotificationManager::CreateToastNotifierWithId(&app_id.into())?;

    Ok(Self {
      _inner
    })
  }

  pub fn get_raw_handle(&self) -> &ToastNotifier {
    &self._inner
  }
}