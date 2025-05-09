use crate::{notification::ToastVisualableXML, ToXML};

use super::VisualElement;

#[allow(non_snake_case)]
/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-progress>
pub struct Progress {
  pub title: Option<String>,
  pub value_string_override: Option<String>,
  status: String,
  value: String,
}

pub enum ProgressValue {
  Percentage(u8),
  Indeterminate,
}

impl ToString for ProgressValue {
  fn to_string(&self) -> String {
    match self {
      ProgressValue::Percentage(x) => format!("{}", x / 100),
      ProgressValue::Indeterminate => "indeterminate".to_string(),
    }
  }
}

impl Progress {
  pub fn create(status_text: &str, value: ProgressValue) -> Self {
    Self::new(None, status_text.into(), value, None)
  }

  pub fn set_title(mut self, title: String) -> Self {
    self.title = Some(title);
    self
  }

  pub fn set_value(mut self, value: ProgressValue) -> Self {
    self.value = value.to_string();
    self
  }

  pub fn override_value_string(mut self, value: String) -> Self {
    self.value_string_override = Some(value);
    self
  }

  pub fn new(
    title: Option<String>,
    status_text: String,
    value: ProgressValue,
    value_string_override: Option<String>,
  ) -> Self {
    Self {
      title,
      status: status_text,
      value: value.to_string(),
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
        <progress {} status={:#?} value={:#?} {} />
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
