use crate::{notification::ToastVisualableXML, ToXML};

use super::{TextOrImageElement, VisualElement};

#[derive(Debug, Clone, Copy)]
pub struct AttributionPlacement;

#[derive(Debug, Clone, Copy, Default)]
pub enum HintStyle {
  Base,
  Title,
  Subtitle,
  CaptionSubtle,
  Bold,
  Italic,
  #[default]
  None
}

impl ToString for HintStyle {
  fn to_string(&self) -> String {
    match self {
      HintStyle::Base => r#"hint-style="base""#.to_string(),
      HintStyle::Title => r#"hint-style="title""#.to_string(),
      HintStyle::Subtitle => r#"hint-style="subtitle""#.to_string(),
      HintStyle::CaptionSubtle => r#"hint-style="captionSubtle""#.to_string(),
      HintStyle::Bold => r#"hint-style="bold""#.to_string(),
      HintStyle::Italic => r#"hint-style="italic""#.to_string(),
      HintStyle::None => "".to_string(),
    }
  }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum HintAlign {
  Right,
  #[default]
  None
}

impl ToString for HintAlign {
  fn to_string(&self) -> String {
    match self {
      HintAlign::Right => r#"hint-align="right""#.to_string(),
      HintAlign::None => "".to_string(),
    }
  }
}

#[allow(non_snake_case)]
#[derive(Default)]
/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-text>
pub struct Text {
  pub id: u64,
  pub lang: Option<String>,
  pub placement: Option<AttributionPlacement>,

  pub style: HintStyle,
  pub align: HintAlign,
  pub body: String,
}

impl TextOrImageElement for Text {}

impl Text {
  pub fn create(id: u64, body: &str) -> Self {
    Self::new(id, None, None, body.into())
  }

  pub fn set_align(mut self, align: HintAlign) -> Self {
    self.align = align;
    self
  }

  pub fn set_style(mut self, style: HintStyle) -> Self {
    self.style = style;
    self
  }

  pub fn set_lang(mut self, lang: String) -> Self {
    self.lang = Some(lang);
    self
  }

  pub fn set_placement(mut self, placement: AttributionPlacement) -> Self {
    self.placement = Some(placement);
    self
  }

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
      ..Default::default()
    }
  }
}

impl VisualElement for Text {}

impl ToastVisualableXML for Text {}

impl ToXML for Text {
  fn to_xml(&self) -> String {
    format!(
      r#"
        <text id="{:#?}" {} {} {} {}>
          {body}
        </text>
      "#,
      self.id,
      self.align.to_string(),
      self.style.to_string(),
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
