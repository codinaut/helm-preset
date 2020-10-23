use serde::Deserialize;
use serde_yaml::Mapping;
use std::collections::HashMap;

mod converter;

#[derive(Debug, Default, PartialEq, Deserialize)]
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
    presets: HashMap<String, Mapping>,
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;
    use serde_yaml::Value::String;
    use std::iter::FromIterator;

    #[test]
    fn deserialize_category_explicit() {
        assert!(matches!(
            &serde_yaml::from_str::<Manifest>(
                r"
                    categories:
                      - name: name-1
                        presets:
                          key-1:
                            a: b
                ",
            )
            .unwrap()
            .categories[0],
            Category::Explicit(e) if e == &ExplicitCategory {
                name: "name-1".to_string(),
                presets: hashmap! {
                    "key-1".to_string() => Mapping::from_iter(hashmap! {
                        String("a".to_string()) => String("b".to_string()),
                    })
                },
            }
        ))
    }

    #[test]
    fn deserialize_category_explicit_no_presets() {
        assert!(matches!(
            &serde_yaml::from_str::<Manifest>(
                r"
                    categories:
                      - name: name-2
                ",
            )
            .unwrap()
            .categories[0],
            Category::Explicit(e) if e == &ExplicitCategory {
                name: "name-2".to_string(),
                presets: HashMap::new(),
            }
        ))
    }

    #[test]
    fn deserialize_category_name_only() {
        assert!(matches!(
            &serde_yaml::from_str::<Manifest>(
                r"
                    categories:
                      - name-3
                ",
            )
            .unwrap()
            .categories[0],
            Category::NameOnly(n) if n == "name-3"
        ))
    }

    #[test]
    fn deserialize_include_path() {
        assert_eq!(
            serde_yaml::from_str::<Manifest>(
                r"
                    includePath: path-4
                ",
            )
            .unwrap(),
            Manifest {
                include_path: Some("path-4".to_string()),
                categories: vec![],
            }
        )
    }

    #[test]
    fn deserialize_include_path_null() {
        assert_eq!(
            serde_yaml::from_str::<Manifest>(
                r"
                    includePath: null
                ",
            )
            .unwrap(),
            Manifest {
                include_path: None,
                categories: vec![],
            }
        )
    }

    #[test]
    fn deserialize_empty() {
        assert_eq!(
            serde_yaml::from_str::<Manifest>("{}").unwrap(),
            Manifest {
                include_path: Some(".".to_string()),
                categories: vec![],
            }
        )
    }
}
