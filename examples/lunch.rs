use std::{path::absolute, thread, time::Duration};

use win32_notif::{
  notification::{actions::{
    action::{ActivationType, AfterActivationBehavior},
    ActionButton, 
  }, audio::{Audio, Src}, header::{Header, HeaderActivationType}, visual::{Image, Placement, Text}, ToastDuration}, NotificationActivatedEventHandler, NotificationBuilder, ToastsNotifier
};

fn main() {
  let path = absolute("./examples/strawberry.jpg").unwrap();
  let path = path.to_string_lossy();

  let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();

  

  let notif = NotificationBuilder::new()
    .audio(Audio::new(Src::Reminder, true, false))
    .header(Header::new("food01", "Order Food", "food", Some(HeaderActivationType::Foreground)))
    .visual(
      Image::create(20, format!("file:///{path}"))
        .with_placement(Placement::Hero)
        .without_image_query()
    )
    .visual(
      Text::create(1, "Would you like to order lunch today?")
    )
    .action(
      ActionButton::create("Yes")
        .set_tooltip("Yes")
        .set_activation_type(ActivationType::Foreground)
        .set_after_activation_behavior(AfterActivationBehavior::PendingUpdate) 
        .set_id("yes")
    )
    .action(
      ActionButton::create("No")
        .set_tooltip("No")
        .set_activation_type(ActivationType::Foreground)
        .set_after_activation_behavior(AfterActivationBehavior::Default)
        .set_id("no")
    )
    .set_duration(ToastDuration::Long)
    .on_activated(NotificationActivatedEventHandler::new(|a, b| {
      println!("Triggered");
      let notification = a.unwrap();
      let args = b.unwrap();

      println!("{args:?}");
      if let Some(x) = args.button_id {
        if &x == "yes" {
          
        }
      }

      Ok(())
    }))
    .build(0, &notifier, "01", "ahq")
    .expect("Unable to build notification");

  _ = notif.show();

  loop {
    thread::sleep(Duration::from_millis(200));
  }
}
