use crate::{notification::ToastVisualableXML, ToXML};

use super::VisualElement;

#[derive(Debug, Clone, Copy)]
pub struct AttributionPlacement;

#[allow(non_snake_case)]
/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-text>
pub struct Text {
  pub id: u64,
  pub lang: Option<String>,
  pub placement: Option<AttributionPlacement>,
  pub body: String,
}

impl Text {
  pub fn new(
    id: u64,
    lang: Option<String>,
    placement: Option<AttributionPlacement>,
    body: String,
  ) -> Self {
    Self {
      id,
      lang,
      placement,
      body,
    }
  }
}

impl VisualElement for Text {}

impl ToastVisualableXML for Text {}

impl ToXML for Text {
  fn to_xml(&self) -> String {
    format!(
      r#"
        <text id="{}" {} {}>
          {body}
        </text>
      "#,
      self.id,
      self
        .lang
        .clone()
        .map_or_else(|| string!(""), |x| format!("lang=\"{x}\"")),
      self
        .placement
        .map_or_else(|| "", |_| "placement=\"attribution\""),
      body = self.body
    )
  }
}
