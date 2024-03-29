use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

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
        out_file.write(v.as_bytes());
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
        out_file.write(v.as_bytes());
    }
}
