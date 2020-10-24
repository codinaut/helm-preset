use serde_yaml::Value;

fn deep_merge(target: &mut &mut Value, substitute: Value) {
    if let Value::Mapping(map) = *target {
        if let Value::Mapping(override_map) = substitute {
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
    fn deep_merge_replaces_non_map_with_non_map_substitute() {
        let mut target = Value::String("value-1".to_string());
        let substitute = Value::String("value-replacement-1".to_string());

        deep_merge(&mut &mut target, substitute.clone());
        assert_eq!(target, substitute)
    }

    #[test]
    fn deep_merge_map_with_empty_map_is_no_op() {
        let mut target = serde_yaml::from_str::<Value>(
            r"
                a: b
            ",
        )
        .unwrap();
        let substitute = Value::Mapping(Mapping::new());
        let conclusion = target.clone();

        deep_merge(&mut &mut target, substitute);
        assert_eq!(target, conclusion)
    }

    #[test]
    fn deep_merge_map_with_map_returns_complement() {
        let mut target = serde_yaml::from_str::<Value>(
            r"
                a: b
            ",
        )
        .unwrap();
        let substitute = serde_yaml::from_str::<Value>(
            r"
                a: c
            ",
        )
        .unwrap();

        deep_merge(&mut &mut target, substitute.clone());
        assert_eq!(target, substitute)
    }
}
