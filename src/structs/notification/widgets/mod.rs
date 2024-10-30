use actions::ActionElement;
use visual::VisualElement;

use crate::ToXML;

use super::{ActionableXML, ToastVisualableXML};

pub mod actions;
pub mod audio;
pub mod commands;
pub mod header;
pub mod visual;

pub struct RawXML {
  raw: String,
}

impl RawXML {
  /// Creates a new instance of `RawXML` that can hold arbitrary String
  /// This is useful when you want to use a widget that is not yet supported
  ///
  /// # Safety
  /// This function is unsafe because it bypasses all the safety that other structs guarantee
  pub unsafe fn new<T: ToString>(raw: T) -> Self {
    Self {
      raw: raw.to_string(),
    }
  }
}

impl ActionElement for RawXML {}

impl ActionableXML for RawXML {}

impl VisualElement for RawXML {}

impl ToastVisualableXML for RawXML {}

impl ToXML for RawXML {
  fn to_xml(&self) -> String {
    self.raw.clone()
  }
}
