use crate::{notification::visual::TextOrImageElement, ToXML};

use super::SubgroupXML;

/// Learn More Here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-subgroup>
pub struct SubGroup {
  elements: Vec<Box<dyn TextOrImageElement>>,
}

impl SubgroupXML for SubGroup {}

impl SubGroup {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn visual<T: TextOrImageElement + 'static>(mut self, element: T) -> Self {
    self.elements.push(Box::new(element));
    self
  }

  pub fn new_from(elements: Vec<Box<dyn TextOrImageElement>>) -> Self {
    Self { elements }
  }
}

impl Default for SubGroup {
  fn default() -> Self {
    Self {
      elements: vec![],
    }
  }
}

impl ToXML for SubGroup {
  fn to_xml(&self) -> String {
    let data = self.elements.iter()
      .map(|x| x.to_xml())
      .collect::<Vec<_>>()
      .join("\n");

    format!("
      <subgroup>
        {data}
      </subgroup>
    ")
  }
}