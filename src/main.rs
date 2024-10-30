use win32_notif::{
  notification::{actions::{action::ActivationType, input::InputType, Input}, header::{Header, HeaderActivationType}, RawXML}, refvar, string, NotificationBuilder, ToastsNotifier
};

fn main() {
  let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();
  
  let notif = NotificationBuilder::new()
    .visual(unsafe { RawXML::new("<text>This is raw XML Text</text>") })
    .build(1, &notifier, "a", "ahq")
    .unwrap();

  notif.show();

  loop {}
}
