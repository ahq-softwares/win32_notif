use crate::ToXML;

/// Learn more about it here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-header>
pub struct Header {
  id: String,
  title: String,
  arguments: String,
  activation_type: String,
}

impl Header {
  pub fn new(
    id: String,
    title: String,
    arguments: String,
    activation_type: Option<ActivationType>,
  ) -> Self {
    Self {
      id,
      title,
      arguments,
      activation_type: activation_type.unwrap_or_default().into(),
    }
  }
}

impl ToXML for Header {
  fn to_xml(&self) -> String {
    format!(
      r#"
      <header title="{}" arguments="{}" id="{}" activationType="{}" />
    "#,
      self.title, self.arguments, self.id, self.activation_type
    )
  }
}

#[derive(Default)]
/// Learn more about it here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-header>
pub enum ActivationType {
  #[default]
  Foreground,
  Protocol,
}

impl Into<String> for ActivationType {
  fn into(self) -> String {
    match self {
      ActivationType::Foreground => "foreground".to_string(),
      ActivationType::Protocol => "protocol".to_string(),
    }
  }
}
