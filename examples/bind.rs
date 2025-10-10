use std::{env::args, thread::sleep, time::Duration};

use win32_notif::{
  notification::{
    actions::{
      action::{ActivationType, AfterActivationBehavior},
      ActionButton,
    },
    group::{Group, SubGroup},
    visual::{
      text::{HintAlign, HintStyle},
      Text,
    },
    Scenario,
  },
  NotificationBuilder, ToastsNotifier,
};

const GUID: u128 = 23885548255760334674942869530154890271u128;

pub fn main() {
  let notifier = ToastsNotifier::new("com.ahqstore.app").unwrap();

  let mut argv = args();

  argv.next();
  argv.next();
  argv.next();

  if let Some(x) = argv.next() {
    let notification = NotificationBuilder::new()
      .set_use_button_style(true)
      .visual(
        Text::create_binded(0, "hi")
          .set_style(HintStyle::Header)
          .set_align(HintAlign::Right),
      )
      .value("hi", "This is binded string")
      .action(
        ActionButton::create("Answer")
          .set_tooltip("Answer")
          .set_id("answer")
          .set_activation_type(ActivationType::Background)
          .set_after_activation_behavior(AfterActivationBehavior::PendingUpdate),
      )
      .build(1, &notifier, "a", "ahq")
      .expect("Error");

    notification.show().expect("Not Sent");
  }

  loop {
    sleep(Duration::from_millis(10));
  }
}
