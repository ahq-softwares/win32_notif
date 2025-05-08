use std::{thread, time::Duration};

use win32_notif::{
  notification::actions::{
    action::{ActivationType, AfterActivationBehavior, HintButtonStyle},
    input::{InputType, Selection},
    ActionButton, Input,
  },
  string, NotificationActivatedEventHandler, NotificationBuilder, NotificationDataSet,
  ToastsNotifier,
};

fn main() {
  let notifier = ToastsNotifier::new("Microsoft.Windows.Explorer").unwrap();

  let notif = NotificationBuilder::new()
    .action(Input::new(
      string!("input"),
      string!("Your name?"),
      InputType::Selection(vec![
        Selection {
          content: string!("AHQ"),
          id: string!("a"),
        },
        Selection {
          content: string!("Other"),
          id: string!("other"),
        },
      ]),
      string!("Enter"),
    ))
    .action(ActionButton::new(
      string!("Hello"),
      string!("yes"),
      ActivationType::Foreground,
      AfterActivationBehavior::Default,
      None,
      string!("input"),
      HintButtonStyle::None,
      string!("Hello"),
      false,
    ))
    .value("prog", "0.01")
    .on_activated(NotificationActivatedEventHandler::new(|a, b| {
      let _notif = a.unwrap();
      let _args = b.unwrap();

      println!("{:#?}", _args);

      Ok(())
    }))
    .build(1, &notifier, "a", "ahq")
    .unwrap();

  let _ = notif.show();

  for i in 0..=100 {
    let data = NotificationDataSet::new().unwrap();

    let _ = data.insert("prog", &format!("{}", i as f64 / 100.0));

    let _ = notifier.update(&data, "ahq", "a");

    thread::sleep(Duration::from_millis(200));
  }

  loop {}
}
