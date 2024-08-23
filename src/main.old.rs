use windows::core::{Interface, HSTRING};
use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::TypedEventHandler;
use windows::UI::Notifications::{
  NotificationData, ToastActivatedEventArgs, ToastNotification, ToastNotificationManager,
};

use win32_notif::refvar;

fn main() {
  println!("{}", refvar!(status));
  let xml = r#"<toast>
        <visual>
            <binding template='ToastGeneric'>
                <text>Hello World</text>
                <progress status="{status}" value="{value}" />
            </binding>
        </visual>
        <actions>
            <action activationType='foreground' arguments='ahq' content='Okay' />
        </actions>
    </toast>"#;

  let doc = XmlDocument::new().unwrap();

  doc.LoadXml(&HSTRING::from(xml)).unwrap();

  let data = NotificationData::new().unwrap();
  let _ = data.SetSequenceNumber(1);

  let val = data.Values().unwrap();
  val.Insert(&"value".into(), &"0.5".into());
  val.Insert(&"status".into(), &"Downloading".into());

  let toast = ToastNotification::CreateToastNotification(&doc).unwrap();
  let _ = toast.SetTag(&"main".into());
  let _ = toast.SetGroup(&"ahq".into());
  let _ = toast.SetData(&data);

  let notifier =
    ToastNotificationManager::CreateToastNotifierWithId(&"com.ahqstore.app".into()).unwrap();

  notifier.Show(&toast).unwrap();
  notifier
    .UpdateWithTagAndGroup(&data, &"main".into(), &"ahq".into())
    .unwrap();

  let handler: TypedEventHandler<ToastNotification, windows::core::IInspectable> =
    TypedEventHandler::new(
      |a: &Option<ToastNotification>, b: &Option<windows::core::IInspectable>| {
        let Some(a) = a else {
          panic!("Panic needed");
        };

        let Some(b) = b else {
          panic!("Panic");
        };

        let cast = b.cast::<ToastActivatedEventArgs>().unwrap();

        println!("Called {}", cast.Arguments().unwrap().to_string());
        Ok(())
      },
    );

  toast.Activated(&handler).unwrap();

  for i in 0..100u32 {
    let data = NotificationData::new().unwrap();

    let val = data.Values().unwrap();
    val
      .Insert(&"value".into(), &format!("0.{i}").into())
      .unwrap();

    data.SetSequenceNumber(i).unwrap();

    let _ = toast.SetData(&data);
    notifier
      .UpdateWithTagAndGroup(&data, &"main".into(), &"ahq".into())
      .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));
  }

  loop {}
}
