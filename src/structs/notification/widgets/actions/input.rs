use crate::{map, notification::ActionableXML, ToXML};

use super::ActionElement;

#[allow(non_snake_case)]
/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-input>
pub struct Input {
  pub id: String,
  pub title: String,
  pub placeHolder: String,
  children: String,
  r#type: String,
}

/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-input>
pub enum InputType {
  Text,
  Selection(Vec<Selection>),
}

impl Input {
  pub fn create_text_input(id: &str, title: &str, place_holder: &str) -> Self {
    Self::new(
      id.into(),
      title.into(),
      InputType::Text,
      place_holder.into(),
    )
  }

  pub fn create_selection_input(
    id: &str,
    title: &str,
    place_holder: &str,
    selections: Vec<Selection>,
  ) -> Self {
    Self {
      id: id.into(),
      title: title.into(),
      r#type: "selection".into(),
      placeHolder: place_holder.into(),
      children: map!(selections),
    }
  }

  pub fn new(id: String, title: String, r#type: InputType, place_holder: String) -> Self {
    let (r#type, ch) = match r#type {
      InputType::Text => ("text", vec![]),
      InputType::Selection(ch) => ("selection", ch),
    };

    Self {
      children: map!(ch),
      id,
      title,
      r#type: r#type.into(),
      placeHolder: place_holder,
    }
  }

  pub fn set_selection(&mut self, children: Vec<Selection>) -> &mut Self {
    self.children = map!(children);
    self
  }
}

impl ActionElement for Input {}

impl ToXML for Input {
  fn to_xml(&self) -> String {
    format!(
      r#"
        <input id={:#?} title={:#?} placeHolderContent={:#?} type={:#?} >
          {}
        </input>
      "#,
      self.id, self.title, self.placeHolder, self.r#type, self.children
    )
  }
}

/// Learn more here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-input>
pub struct Selection {
  pub id: String,
  pub content: String,
}

impl ToXML for Selection {
  fn to_xml(&self) -> String {
    format!(
      r#"<selection id={:#?} content={:#?} />"#,
      &self.id, &self.content
    )
  }
}

impl ActionableXML for Input {}
