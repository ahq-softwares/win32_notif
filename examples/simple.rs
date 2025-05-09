use win32_notif::{
  notification::{
    actions::ActionButton,
    group::{Group, SubGroup},
    visual::{
      text::{HintAlign, HintStyle},
      Text,
    },
    Scenario,
  },
  NotificationBuilder, ToastsNotifier,
};

pub fn main() {
  let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();

  let notification = NotificationBuilder::new()
    .set_scenario(Scenario::IncomingCall)
    .set_use_button_style(true)
    .visual(
      Group::new()
        .subgroup(
          SubGroup::new().visual(Text::create(0, "Hello World").set_style(HintStyle::Title)),
        )
        .subgroup(
          SubGroup::new().visual(
            Text::create(0, "Hello World x2")
              .set_style(HintStyle::Header)
              .set_align(HintAlign::Right),
          ),
        ),
    )
    .action(
      ActionButton::create("Answer")
        .set_tooltip("Answer")
        .set_id("answer"),
    )
    .build(1, &notifier, "a", "ahq")
    .expect("Error");

  notification.show().expect("Not Sent");
}
