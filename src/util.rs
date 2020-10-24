use serde_yaml::{Mapping, Value};

fn merge_map(target: &mut Mapping, substitute: Mapping) {
    for (k, v) in substitute {
        target.insert(k, v);
    }
}

pub fn deep_merge(target: &mut &mut Value, substitute: Value) {
    match (target, substitute) {
        (Value::Mapping(target_map), Value::Mapping(substitute_map)) => {
            merge_map(target_map, substitute_map)
        }
        (target_any, substitute_any) => {
            **target_any = substitute_any
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_yaml::Mapping;

    #[test]
    fn deep_merge_non_map_with_non_map_returns_substituted() {
        let mut target = Value::String("value-1".to_string());
        let substitute = Value::String("value-replacement-1".to_string());

        deep_merge(&mut &mut target, substitute.clone());
        assert_eq!(target, substitute)
    }

    #[test]
    fn deep_merge_map_with_complement_empty_map_is_no_op() {
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
    fn deep_merge_map_with_substitute_map_returns_substituted() {
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
