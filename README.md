# Win32 Notif

[![Crates.io Version](https://img.shields.io/crates/v/win32_notif?logo=Docs.rs)](https://docs.rs/win32_notif)

A lightweight crate to help you to compose beautiful notifications for Windows OS.

This crate aims for **100%** coverage of the WinRT Toast api as much as possible.

Thankfully we are quite near that goal due to our unique approach to notification content: **widgets**

You declare your own style, however you like as long as the XML Supports it.

## Basic Usage

```rust
use win32_notif::{Notif, NotifIcon, NotifState, NotifType, NotifFlags};

fn main() {
  let notifier = ToastsNotifier::new("windows app user model id").unwrap();

  // Not correct, undergoing massive rewrite
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

## What is implemented

We've actually implemented a lot of the Notification APIs

### Containers

- Text
- Image
- Progressbar
- Groups
- Subgroups

### Handlers

- Foreground OnActivated
- Foreground OnError
- Foregrounf OnDismissed

### Utility

- Notification Updating
- Data Binding (so that you can update notification content)
- Notification Duration
- Scenarios
- Command
- Actions
- Inputs
- Selections
- Visual

and a lot of other things... 🎉

## Future Project Plan

- COM Activator
- Background Activation Handling

...and that's it
