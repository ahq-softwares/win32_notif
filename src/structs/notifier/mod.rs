use std::sync::Arc;

use windows::{
  core::HSTRING, 
  UI::Notifications::{
    NotificationData, NotificationUpdateResult, ToastNotificationHistory, ToastNotificationManager, ToastNotifier
  }
};

use crate::{NotifError, notification::OwnedPartialNotification};

use super::NotificationDataSet;

pub struct ToastsNotifier {
  _inner: ToastNotifier,
  app_id: Arc<Box<str>>
}

impl ToastsNotifier {
  pub fn new<T: Into<String>>(app_id: T) -> Result<Self, NotifError> {
    let string: String = app_id.into();
    let string = string.into_boxed_str();

    let id = HSTRING::from(string.as_ref());
    let _inner = ToastNotificationManager::CreateToastNotifierWithId(&id)?;

    Ok(Self { _inner, app_id: Arc::new(string) })
  }

  pub fn manager(&self) -> Result<ToastsManager, NotifError> {
    Ok(ToastsManager {
      inner: (ToastNotificationManager::History()?),
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

#[derive(Debug, Clone)]
pub struct ToastsManager {
  pub(crate) inner: ToastNotificationHistory,
  pub app_id: Arc<Box<str>>
}

impl ToastsManager {
  #[cfg(feature = "unsafe")]
  pub unsafe fn inner(&self) -> &ToastNotificationHistory {
    &self.inner
  }

  /// Clear all notifications from this application
  pub fn clear(&self) -> Result<(), NotifError> {
    Ok(self.inner.Clear()?)
  }

  /// Clears all notifications sent by another app
  /// from the same app package
  /// 
  /// ## WARNING
  /// This is probably not meant for Win32 Apps but we're not sure
  pub fn clear_appid(&self, app_id: &str) -> Result<(), NotifError> {
    let hstr = HSTRING::from(app_id);

    Ok(self.inner.ClearWithId(&hstr)?)
  }

  /// Removes a notification identified by tag, group, notif_id
  pub fn remove_notification(&self, tag: &str, group: &str, notif_id: &str) -> Result<(), NotifError> {
    let hstr = HSTRING::from(tag);
    let group = HSTRING::from(group);
    let id = HSTRING::from(notif_id);

    Ok(self.inner.RemoveGroupedTagWithId(&hstr, &group, &id)?)
  }

  /// Removes a notification identified by tag, group
  pub fn remove_notification_with_gt(&self, tag: &str, group: &str) -> Result<(), NotifError> {
    let hstr = HSTRING::from(tag);
    let group = HSTRING::from(group);

    Ok(self.inner.RemoveGroupedTag(&hstr, &group)?)
  }

  
  /// Removes a notification identified by tag only
  pub fn remove_notification_with_tag(&self, tag: &str) -> Result<(), NotifError> {
    let hstr = HSTRING::from(tag);

    Ok(self.inner.Remove(&hstr)?)
  }

  /// Removes a group of notifications identified by the group id
  pub fn remove_group(&self, group: &str) -> Result<(), NotifError> {
    let hstr = HSTRING::from(group);

    Ok(self.inner.RemoveGroup(&hstr)?)
  }

  /// Removes a group of notifications identified by the group id for **another app**
  /// from the same app package
  /// 
  /// ## WARNING
  /// This is probably not meant for Win32 Apps but we're not sure
  pub fn remove_group_from_appid(&self, group: &str, app_id: &str) -> Result<(), NotifError> {
    let app_id = HSTRING::from(app_id);
    let hstr = HSTRING::from(group);

    Ok(self.inner.RemoveGroupWithId(&hstr, &app_id)?)
  }

  /// Gets notification history as PartialNotification objects
  pub fn get_notification_history(&self) -> Result<Vec<OwnedPartialNotification>, NotifError> {
    let data = self.inner.GetHistory()?;

    let da = data.into_iter().map(|x| OwnedPartialNotification { notif: x }).collect::<Vec<_>>();

    Ok(da)
  }

  /// Gets notification history as PartialNotification objects for **another app**
  /// from the same app package
  /// 
  /// ## WARNING
  /// This is probably not meant for Win32 Apps but we're not sure
  pub fn get_notification_history_with_id(&self, app_id: &str) -> Result<Vec<OwnedPartialNotification>, NotifError> {
    let appid = HSTRING::from(app_id);

    let data = self.inner.GetHistoryWithId(&appid)?;

    let da = data.into_iter().map(|x| OwnedPartialNotification { notif: x }).collect::<Vec<_>>();

    Ok(da)
  }
}