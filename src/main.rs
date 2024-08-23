use win32_notif::{
  notification::{actions::{action::ActivationType, input::InputType, Input}, header::{Header, HeaderActivationType}},
  string, NotificationBuilder, ToastsNotifier,
};

fn main() {
  let notifier = ToastsNotifier::new("com.ahqstore.app").unwrap();
  
  let notif = NotificationBuilder::new()
    .header(Header::new("ahq", "Important Notification", "arg",Some(HeaderActivationType::Foreground)))
    .action(Input::new(
      string!("test"),
      string!(""),
      InputType::Text,
      string!("Ohk?"),
    ))
    .build(1, &notifier, "a", "ahq")
    .unwrap();

  notif.show();
}
