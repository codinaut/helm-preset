use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    #[serde(default = "default_include_path")]
    include_path: Option<String>,

    #[serde(default)]
    categories: Vec<Category>,
}

fn default_include_path() -> Option<String> {
    Some(".".to_string())
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Category {
    NameOnly(String),
    Explicit(ExplicitCategory),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ExplicitCategory {
    name: String,

    #[serde(default)]
    presets: HashMap<String, Box<Value>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{faker::lorem, Fake};
    use maplit::hashmap;

    #[test]
    fn deserialize_category_explicit() {
        let name = lorem::en::Word().fake();
        let preset_key: String = lorem::en::Word().fake();
        let preset_value: String = lorem::en::Word().fake();
        assert!(matches!(
            serde_yaml::from_str::<Category>(&format!(
                r#"
                name: "{}"
                presets:
                  {}: {}
            "#,
                name, preset_key, preset_value
            ))
            .unwrap(),
            Category::Explicit(e) if e == ExplicitCategory {
                name,
                presets: hashmap! {
                    preset_key => Box::new(Value::String(preset_value))
                },
            }
        ))
    }

    #[test]
    fn deserialize_category_explicit_no_presets() {
        let name = lorem::en::Word().fake();
        assert!(matches!(
            serde_yaml::from_str::<Category>(&format!(
                r#"
                name: "{}"
            "#,
                name
            ))
            .unwrap(),
            Category::Explicit(e) if e == ExplicitCategory {
                name,
                presets: HashMap::new()
            }
        ))
    }

    #[test]
    fn deserialize_category_name_only() {
        let name: String = lorem::en::Word().fake();
        assert!(matches!(
            serde_yaml::from_str::<Category>(&format!(
                r#"
                "{}"
            "#,
                name
            ))
            .unwrap(),
            Category::NameOnly(n) if n == name
        ))
    }

    #[test]
    fn deserialize_manifest_include_path() {
        let include_path = lorem::en::Word().fake();
        assert_eq!(
            serde_yaml::from_str::<Manifest>(&format!(
                r#"
                includePath: {}
                "#,
                include_path
            ))
            .unwrap(),
            Manifest {
                include_path: Some(include_path),
                categories: vec![],
            }
        );
    }

    #[test]
    fn deserialize_manifest_include_path_null() {
        assert_eq!(
            serde_yaml::from_str::<Manifest>(
                r#"
                includePath: null
                "#,
            )
            .unwrap(),
            Manifest {
                include_path: None,
                categories: vec![],
            }
        );
    }

    #[test]
    fn deserialize_manifest_empty() {
        assert_eq!(
            serde_yaml::from_str::<Manifest>("{}")
            .unwrap(),
            Manifest {
                include_path: Some(".".to_string()),
                categories: vec![],
            }
        );
    }
}
