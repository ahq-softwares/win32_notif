use std::{thread, time::Duration};

use win32_notif::{
  notification::{actions::{
    action::{ActivationType, AfterActivationBehavior, HintButtonStyle},
    input::{InputType, Selection},
    ActionButton, Input,
  }, group::{Group, SubGroup}, visual::{text::{HintAlign, HintStyle}, Text}},
  string, NotificationActivatedEventHandler, NotificationBuilder, NotificationDataSet,
  ToastsNotifier,
};

pub fn main() {
  let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();

  let notification = NotificationBuilder::new()
    .visual(
      Group::new()
        .subgroup(
          SubGroup::new()
          .visual(Text::create(0, "Hello World").set_style(HintStyle::Base))
        )
        .subgroup(
          SubGroup::new()
          .visual(Text::create(0, "Hello World x2").set_style(HintStyle::CaptionSubtle).set_align(HintAlign::Right))
        )
    )
    .build(1, &notifier, "a", "ahq")
    .expect("Error");

  notification.show().expect("Not Sent");
}