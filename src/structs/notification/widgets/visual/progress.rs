use crate::{notification::ToastVisualableXML, ToXML};

use super::VisualElement;

#[allow(non_snake_case)]
/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-progress>
pub struct Progress {
  pub title: Option<String>,
  pub status: String,
  pub value: String,
  pub value_string_override: Option<String>,
}

impl Progress {
  pub fn new(
    title: Option<String>,
    status: String,
    value: String,
    value_string_override: Option<String>,
  ) -> Self {
    Self {
      title,
      status,
      value,
      value_string_override,
    }
  }
}

impl VisualElement for Progress {}

impl ToastVisualableXML for Progress {}

impl ToXML for Progress {
  fn to_xml(&self) -> String {
    format!(
      r#"
        <progress {} status="{}" value="{}" {} />
      "#,
      self
        .title
        .clone()
        .map_or_else(|| string!(""), |x| format!("title=\"{x}\"")),
      self.status,
      self.value,
      self
        .value_string_override
        .clone()
        .map_or_else(|| string!(""), |x| format!("valueStringOverride=\"{x}\""))
    )
  }
}
