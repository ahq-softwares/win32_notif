use crate::{notification::ActionableXML, ToXML};

use super::ActionElement;

#[allow(non_snake_case)]
/// Learn More Here
/// <https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-action>
pub struct ActionButton {
  pub content: String,
  pub arguments: String,
  pub imageUri: Option<String>,
  pub hint_inputid: String,
  pub hint_toolTip: String,

  activationType: String,
  afterActivationBehavior: String,
  hint_buttonStyle: String,
  placement: bool,
}

#[allow(non_snake_case)]
impl ActionButton {
  pub fn create(content: &str) -> Self {
    Self::new(
      content.into(),
      content.into(),
      ActivationType::Foreground,
      AfterActivationBehavior::Default,
      None,
      "".into(),
      HintButtonStyle::None,
      "".into(),
      false,
    )
  }

  pub fn set_id(mut self, id: &str) -> Self {
    self.arguments = id.into();
    self
  }

  /// Provide input id to place the button near an input
  pub fn set_input_id(mut self, id: &str) -> Self {
    self.hint_inputid = id.into();
    self
  }

  pub fn set_tooltip(mut self, tooltip: &str) -> Self {
    self.hint_toolTip = tooltip.into();
    self
  }

  pub fn set_image_uri(mut self, uri: &str) -> Self {
    self.imageUri = Some(uri.into());
    self
  }

  pub fn set_context_menu_placement(mut self, enabled: bool) -> Self {
    self.placement = enabled;
    self
  }

  pub fn set_activation_type(mut self, activation_type: ActivationType) -> Self {
    self.activationType = activation_type.into();
    self
  }

  pub fn set_after_activation_behavior(
    mut self,
    after_activation_behavior: AfterActivationBehavior,
  ) -> Self {
    self.afterActivationBehavior = after_activation_behavior.into();
    self
  }

  pub fn set_button_style(mut self, hint_buttonStyle: HintButtonStyle) -> Self {
    self.hint_buttonStyle = hint_buttonStyle.into();
    self
  }

  pub fn set_content(mut self, content: &str) -> Self {
    self.content = content.into();
    self
  }

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
          <action content={:#?} arguments={:#?} activationType={:#?} afterActivationBehavior={:#?} imageUri={:#?} hint-inputId={:#?} hint-buttonStyle={:#?} hint-toolTip={:#?} {} />
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
