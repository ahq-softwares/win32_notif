use win32_notif::{
  notification::actions::{input::InputType, Input},
  string, NotificationBuilder,
};

fn main() {
  NotificationBuilder::new()
    .action(Input::new(
      string!("test"),
      string!(""),
      InputType::Text,
      string!("Ohk?"),
    ))
    .build(1);
}
