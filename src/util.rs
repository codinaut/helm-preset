use serde_yaml::Value;

fn deep_merge(target: &mut &mut Value, override_value: Value) {
    if let Value::Mapping(map) = *target {
        if let Value::Mapping(override_map) = override_value {
            for (k, v) in override_map {
                map.insert(k, v);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_yaml::Mapping;

    #[test]
    fn merge_with_empty() {
        let target = serde_yaml::from_str::<Value>(
            r"
                a: b
            ",
        )
        .unwrap();
        let mut source = target.clone();
        deep_merge(&mut &mut source, Value::Mapping(Mapping::new()));
        assert_eq!(source, target)
    }

    #[test]
    fn merge_with_replacement() {
        let mut source = serde_yaml::from_str::<Value>(
            r"
                a: b
            ",
        )
        .unwrap();
        let target = serde_yaml::from_str::<Value>(
            r"
                a: c
            ",
        )
        .unwrap();
        deep_merge(&mut &mut source, target.clone());
        assert_eq!(source, target)
    }
}
