use hashbrown::HashMap;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDefinition {
    pub groups: Option<Vec<ComponentDefinitionItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDefinitionItem {
    pub title: Option<String>,
    pub id: Option<String>,
    pub components: Option<Vec<ComponentDefinitionItemComponent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDefinitionItemComponent {
    pub title: Option<String>,
    pub id: Option<String>,
    pub plugins: Option<HashMap<String, PluginDefinition>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDefinition {
    pub page: XwalkPluginPage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XwalkPluginPage {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub template: Option<HashMap<String, String>>,
}

// Component Filters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentFilters(pub Vec<ComponentFilter>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentFilter {
    pub id: String,
    pub components: Vec<String>,
}

// Component models
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ComponentModels(pub Vec<ComponentModel>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentModel {
    pub id: String,
    pub fields: Vec<ComponentField>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ComponentField {
    Container(ContainerField),
    TextInput(TextInputField),
    Select(SelectField),
    MultiSelect(MultiSelectField),
    Reference(ReferenceField),
    RichText(RichTextField),
    TextArea(TextAreaField),
}

impl<'de> Deserialize<'de> for ComponentField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        if let Some(component_type) = value.get("component").and_then(Value::as_str) {
            let result = match component_type {
                "container" => ContainerField::deserialize(value).map(ComponentField::Container),
                "text" | "text-input" => {
                    TextInputField::deserialize(value).map(ComponentField::TextInput)
                }
                "select" => SelectField::deserialize(value).map(ComponentField::Select),
                "reference" => ReferenceField::deserialize(value).map(ComponentField::Reference),
                "rich-text" | "richtext" => {
                    RichTextField::deserialize(value).map(ComponentField::RichText)
                }
                "text-area" => TextAreaField::deserialize(value).map(ComponentField::TextArea),
                "multiselect" => {
                    MultiSelectField::deserialize(value).map(ComponentField::MultiSelect)
                }
                _ => Err(serde::de::Error::custom(format!(
                    "unknown component type: {}",
                    component_type
                ))),
            };

            result.map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom("Missing component field"))
        }
    }
}

fn deserialize_component_field<'de, D>(deserializer: D) -> Result<ComponentField, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    if let Some(component_type) = value.get("component").and_then(Value::as_str) {
        let result = match component_type {
            "container" => ContainerField::deserialize(value).map(ComponentField::Container),
            "text" | "text-input" => {
                TextInputField::deserialize(value).map(ComponentField::TextInput)
            }
            "select" => SelectField::deserialize(value).map(ComponentField::Select),
            "reference" => ReferenceField::deserialize(value).map(ComponentField::Reference),
            "rich-text" | "richtext" => {
                RichTextField::deserialize(value).map(ComponentField::RichText)
            }
            _ => Err(serde::de::Error::custom(format!(
                "unknown component type: {}",
                component_type
            ))),
        };

        result.map_err(serde::de::Error::custom)
    } else {
        Err(serde::de::Error::custom("Missing component field"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "aem-tag")]
    AemTag,
    #[serde(rename = "aem-content")]
    AemContent,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "checkbox-group")]
    CheckboxGroup,
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "multiselect")]
    MultiSelect,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "radio-group")]
    RadioGroup,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "rich-text", alias = "richtext")]
    RichText,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "text", alias = "text-input")]
    Text,
    #[serde(rename = "text-area")]
    TextArea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldLabel(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDescription(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueType(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesLogic(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationType(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseField {
    pub component: ComponentType,
    pub name: String,
    pub label: FieldLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<FieldDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<FieldValue>,
    #[serde(rename = "valueType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<ValueType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<RulesLogic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextInputField {
    #[serde(flatten)]
    pub base_field: BaseField,
    #[serde(rename = "minLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(rename = "maxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(rename = "regExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customErrorMsg")]
    pub custom_error_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextField {
    #[serde(flatten)]
    pub base_field: BaseField,
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u32>,
    #[serde(rename = "customErrorMsg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_error_msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectField {
    #[serde(flatten)]
    pub base_field: BaseField,
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u32>,
    pub options: Option<Vec<SelectOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<SelectOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextAreaField {
    #[serde(flatten)]
    pub base_field: BaseField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceField {
    #[serde(flatten)]
    pub base_field: BaseField,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSelectField {
    #[serde(flatten)]
    pub base_field: BaseField,
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<u32>,
    pub options: Option<Vec<SelectOption>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerField {
    #[serde(flatten)]
    pub base_field: BaseField,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsible: Option<bool>,
}

#[cfg(test)]
pub mod tests {
    use std::fs::{self, OpenOptions};
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn it_deserializes_component_definition() {
        let filename = "test-resources/component-definition.json";
        let data = fs::read_to_string(filename).unwrap();

        let component_definition: ComponentDefinition = serde_json::from_str(&data).unwrap();

        assert_eq!(
            &component_definition
                .clone()
                .groups
                .unwrap()
                .first()
                .unwrap()
                .title
                .clone()
                .unwrap(),
            "Default Content"
        );

        let v = serde_json::to_string_pretty(&component_definition).unwrap();

        let mut out_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("test-resources/component-definition.json.new")
            .unwrap();
        let _ = out_file.write_all(v.as_bytes());
    }

    #[test]
    fn it_deserializes_component_filters() {
        let filename = "test-resources/component-filters.json";
        let data = fs::read_to_string(filename).unwrap();

        let component_filters: ComponentFilters = serde_json::from_str(&data).unwrap();

        assert_eq!(
            &component_filters.clone().0.first().unwrap().id.clone(),
            "main"
        );

        let v = serde_json::to_string_pretty(&component_filters).unwrap();

        let mut out_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("test-resources/component-filters.json.new")
            .unwrap();
        let _res = out_file.write_all(v.as_bytes());
    }

    #[test]
    fn it_deserializes_component_models() {
        let filename = "test-resources/component-models.json";
        let data = fs::read_to_string(filename).unwrap();

        let component_models: ComponentModels = serde_json::from_str(&data).unwrap();

        assert_eq!(
            &component_models.clone().0.first().unwrap().id.clone(),
            "title"
        );

        let v = serde_json::to_string_pretty(&component_models).unwrap();

        let mut out_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("test-resources/component-models.json.new")
            .unwrap();
        let _res = out_file.write_all(v.as_bytes());
    }
}
