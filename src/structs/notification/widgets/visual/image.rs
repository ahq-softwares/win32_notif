use crate::{notification::ToastVisualableXML, ToXML};

use super::VisualElement;

/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-image#attributes>
pub enum Placement {
  AppLogoOverride,
  Hero,
  None,
}

impl ToString for Placement {
  fn to_string(&self) -> String {
    match self {
      Placement::AppLogoOverride => "placement=\"appLogoOverride\"".to_string(),
      Placement::Hero => "placement=\"hero\"".to_string(),
      Placement::None => "".to_string(),
    }
  }
}

#[allow(non_snake_case)]
/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-image>
pub struct Image {
  pub id: u64,
  pub src: String,
  pub alt: Option<String>,
  pub add_image_query: bool,
  pub placement: Placement,
  pub crop_circle: bool,
}

impl Image {
  pub fn new(
    id: u64,
    src: String,
    alt: Option<String>,
    add_image_query: bool,
    placement: Placement,
    crop_circle: bool,
  ) -> Self {
    Self {
      id,
      add_image_query,
      src,
      alt,
      placement,
      crop_circle,
    }
  }
}

impl VisualElement for Image {}

impl ToastVisualableXML for Image {}

impl ToXML for Image {
  fn to_xml(&self) -> String {
    format!(
      r#"
        <image id="{id}" src="{src}" {alt} addImageQuery="{add_image_query}" {placement} {crop} />
      "#,
      id = self.id,
      src = &self.src,
      alt = self
        .alt
        .clone()
        .map_or_else(|| string!(""), |x| format!("alt=\"{x}\"")),
      add_image_query = self.add_image_query,
      placement = self.placement.to_string(),
      crop = if self.crop_circle {
        "hint-crop=\"circle\""
      } else {
        ""
      }
    )
  }
}
