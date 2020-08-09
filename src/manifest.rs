use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Category {
    name: String,
    presets: HashMap<String, Box<Value>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::{faker::lorem, Fake};
    use maplit::hashmap;

    #[test]
    fn deserialize_category() {
        let name = lorem::en::Word().fake();
        let preset_key = lorem::en::Word().fake();
        let preset_value = lorem::en::Word().fake();
        assert_eq!(
            serde_yaml::from_str::<Category>(&format!(
                r#"
                name: "{}"
                presets:
                  {}: {}
            "#,
                name, preset_key, preset_value
            ))
            .unwrap(),
            Category {
                name,
                presets: hashmap! {
                    preset_key => Box::new(Value::String(preset_value))
                },
            }
        )
    }
}
