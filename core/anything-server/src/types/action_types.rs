use std::collections::HashMap;

use super::react_flow_types::{HandleProps, NodePresentation};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ValidationFieldType {
    String,
    Number,
    Object,
    Boolean,
    Array,
    Null,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InputFieldType {
    SimpleText,
    NumberOrVariable,
    BooleanOrVariable,
    ObjectOrVariable,
    HtmlOrVariable,
    XmlOrVariable,
    SelectOrVariable,
    Text,
    Account,
    Error,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ValidationField {
    pub r#type: ValidationFieldType,
}

impl Default for ValidationField {
    fn default() -> Self {
        ValidationField {
            r#type: ValidationFieldType::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PresentationField {
    #[serde(rename = "inputType")]
    pub input_type: InputFieldType,
}

impl Default for PresentationField {
    fn default() -> Self {
        PresentationField {
            input_type: InputFieldType::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchemaProperty {
    #[serde(rename = "x-any-validation")]
    pub x_any_validation: Option<ValidationField>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    #[serde(rename = "oneOf")]
    //Used for select fields
    pub one_of: Option<Vec<serde_json::Value>>,
    #[serde(rename = "x-jsf-presentation")]
    pub x_jsf_presentation: Option<PresentationField>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonSchema {
    pub r#type: Option<String>,
    pub properties: Option<HashMap<String, JsonSchemaProperty>>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "allOf")]
    pub all_of: Option<Vec<serde_json::Value>>,
    #[serde(rename = "x-jsf-order")]
    pub x_jsf_order: Option<Vec<String>>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    pub anything_action_version: String,
    pub r#type: ActionType,
    pub plugin_id: String,
    pub action_id: String,
    pub plugin_version: String,
    pub label: String,
    pub description: Option<String>,
    pub icon: String,
    pub variables: Option<Value>,
    pub variables_locked: Option<bool>,
    pub variables_schema: Option<JsonSchema>,
    pub variables_schema_locked: Option<bool>,
    pub input: Value,
    pub input_locked: Option<bool>,
    pub input_schema: JsonSchema,
    pub input_schema_locked: Option<bool>,
    pub presentation: Option<NodePresentation>,
    pub handles: Option<Vec<HandleProps>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    Trigger,  // Trigger action
    Action,   // General action
    Loop,     // Loop action
    Decision, // Decision action
    Filter,   // Filter action
    Response, // Response action for making api endpoints
    Input,    // Input action for subflows
    Output,   // Output action for subflows
}

impl ActionType {
    pub fn as_str(&self) -> &str {
        match self {
            ActionType::Input => "input",
            ActionType::Trigger => "trigger",
            ActionType::Response => "response",
            ActionType::Action => "action",
            ActionType::Loop => "loop",
            ActionType::Decision => "decision",
            ActionType::Filter => "filter",
            ActionType::Output => "output",
        }
    }
}
