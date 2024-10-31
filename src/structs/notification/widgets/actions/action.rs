use crate::{notification::ActionableXML, ToXML};

use super::ActionElement;

#[allow(non_snake_case)]
/// Learn More Here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-action>
pub struct ActionButton {
  pub content: String,
  pub arguments: String,
  pub activationType: String,
  pub afterActivationBehavior: String,
  pub imageUri: Option<String>,
  pub hint_inputid: String,
  pub hint_buttonStyle: String,
  pub hint_toolTip: String,
  pub placement: bool,
}

#[allow(non_snake_case)]
impl ActionButton {
  pub fn new(
    content: String,
    arguments: String,
    activation_type: ActivationType,
    after_activation_behavior: AfterActivationBehavior,
    image_uri: Option<String>,
    hint_inputid: String,
    hint_buttonStyle: HintButtonStyle,
    hint_toolTip: String,
    placement: bool,
  ) -> Self {
    Self {
      content,
      arguments,
      activationType: activation_type.into(),
      afterActivationBehavior: after_activation_behavior.into(),
      imageUri: image_uri,
      hint_inputid,
      hint_buttonStyle: hint_buttonStyle.into(),
      hint_toolTip,
      placement,
    }
  }
}

impl ToXML for ActionButton {
  fn to_xml(&self) -> String {
    format!(
      r#"
          <action content="{}" arguments="{}" activationType="{}" afterActivationBehavior="{}" imageUri="{}" hint-inputId="{}" hint-buttonStyle="{}" hint-toolTip="{}" {} />
        "#,
      self.content,
      self.arguments,
      self.activationType,
      self.afterActivationBehavior,
      self.imageUri.as_ref().unwrap_or(&"".to_string()),
      self.hint_inputid,
      self.hint_buttonStyle,
      self.hint_toolTip,
      if self.placement {
        "placement=\"contextMenu\""
      } else {
        ""
      }
    )
  }
}

#[derive(Default)]
/// Learn More Here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-action>
pub enum ActivationType {
  #[default]
  Foreground,
  Background,
  Protocol,
}

impl Into<String> for ActivationType {
  fn into(self) -> String {
    match self {
      ActivationType::Foreground => "foreground".to_string(),
      ActivationType::Background => "background".to_string(),
      ActivationType::Protocol => "protocol".to_string(),
    }
  }
}

#[derive(Default)]
/// Learn More Here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-action>
pub enum AfterActivationBehavior {
  #[default]
  Default,
  PendingUpdate,
}

impl Into<String> for AfterActivationBehavior {
  fn into(self) -> String {
    match self {
      Self::Default => "default".to_string(),
      Self::PendingUpdate => "pendingUpdate".to_string(),
    }
  }
}

#[derive(Default)]
/// Learn More Here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-action>
pub enum HintButtonStyle {
  #[default]
  None,
  Success,
  Critical,
}

impl Into<String> for HintButtonStyle {
  fn into(self) -> String {
    match self {
      Self::None => "".to_string(),
      Self::Success => "success".to_string(),
      Self::Critical => "critical".to_string(),
    }
  }
}

impl ActionElement for ActionButton {}
impl ActionableXML for ActionButton {}
