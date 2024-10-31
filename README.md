# Win32 Notif

A lightweight crate to help you to compose beautiful notifications for Windows OS.

## Usage

```rust
use win32_notif::{Notif, NotifIcon, NotifState, NotifType, NotifFlags};

fn main() {
  let notifier = ToastsNotifier::new("windows app user model id").unwrap();

  let notif = NotificationBuilder::new()
    .visual(Text::new(2, None, None, string!("Hello There 👋🏼")))
    .action(ActionButton::new(
      string!("Yes"),
      string!("yes"),
      ActivationType::Foreground,
      AfterActivationBehavior::Default,
      None,
      string!("yes"),
      HintButtonStyle::Success,
      string!("Yes"),
      false
    ))
    .build(2, &*NOTIFIER, "tag", "group")
    .unwrap();

  notif.show().unwrap();
}
```
