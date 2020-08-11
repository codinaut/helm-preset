use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Manifest {
    includes: Vec<String>,
    categories: Vec<Category>,
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
    fn deserialize_manifest() {
        let includes = vec![lorem::en::Word().fake(), lorem::en::Word().fake()];
        assert_eq!(
            serde_yaml::from_str::<Manifest>(&format!(
                r#"
                includes:
                  - {}
                  - {}
                categories: []
                "#,
                includes[0], includes[1]
            ))
            .unwrap(),
            Manifest {
                includes,
                categories: vec![],
            }
        );
    }
}
